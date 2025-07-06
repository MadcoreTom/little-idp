use actix_web::http::header::{self, ContentType};
use actix_web::{HttpResponse, Responder};
use bytes::Bytes;
use std::collections::HashMap;
use url::{Url, form_urlencoded};

use crate::auth_code::{create_auth_code, format_auth_code};
use crate::db::{self, add_auth_code};

static LOGIN_HTML: &str = include_str!("login.html");

pub async fn login_form() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(LOGIN_HTML)
}

pub async fn login_submit(body: Bytes) -> HttpResponse {
    // Parse formdata body into map
    let form_data: HashMap<String, String> = parse_form_data(body);
    // parse callback url from url
    let callback: String = form_data["callback"].to_string();

    // So in here we verify the username and password
    // Then make a time-limited token
    // and include it in a redirect
    let pass = db::get_user_pass(form_data["user"].clone());

    let resp: String;
    match pass {
        Some(hash) => match bcrypt::verify(&form_data["pass"], &hash) {
            Ok(valid) => {
                if valid {
                    return handle_successful_auth(callback, form_data["user"].clone());
                } else {
                    println!("Wrong password");
                    resp = "oh noes".to_string();
                }
            }
            Err(_) => {
                println!("Unable to verify password hash");
                resp = LOGIN_HTML.to_string();
            }
        },
        None => {
            println!("User {} not found", form_data["user"]);
            resp = "ERR".to_string()
        },
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(resp)
}

fn handle_successful_auth(callback: String, user: String) -> HttpResponse {
    let auth_code = create_auth_code();
    add_auth_code(auth_code.key.clone(), auth_code.secret_hash.clone(), user);

    println!("Callback {}", callback);
    // TODO parse URL first before even trying to login
    let url_result = Url::parse(&callback);
    match url_result {
        Ok(mut url) => {
            url.query_pairs_mut().append_pair(
                "code",
                format_auth_code(&auth_code).as_str(),
            );
            return HttpResponse::Found()
                .append_header((header::LOCATION, url.to_string()))
                .finish();
        }
        Err(err) => {
            println!("Failed to parse callback url");
            return HttpResponse::InternalServerError() // TODO reconsider error code
                .content_type("text/plain")
                .body(format!("bad callback: {}", err.to_string()));
        }
    }
}

fn parse_form_data(body: Bytes) -> HashMap<String, String> {
    let mut form_data: HashMap<String, String> = HashMap::new();
    for (key, value) in form_urlencoded::parse(body.as_ref()) {
        form_data.insert(key.into_owned(), value.into_owned());
    }
    form_data
}