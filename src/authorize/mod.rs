use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, Responder};
use bytes::Bytes;
use std::collections::HashMap;
use url::form_urlencoded;

use crate::auth_code::create_auth_code;
use crate::db::{self, add_auth_code};

static LOGIN_HTML: &str = include_str!("login.html");

pub async fn login_form() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(LOGIN_HTML)
}

pub async fn login_submit(body: Bytes) -> impl Responder {
    // Parse formdata body into map
    let form_data: HashMap<String, String> = parse_form_data(body);

    // So in here we verify the username and password
    // Then make a time-limited token
    // and include it in a redirect
    let pass = db::get_user_pass(form_data["user"].clone());

    let resp: String;
    match pass {
        Some(hash) => match bcrypt::verify(&form_data["pass"], &hash) {
            Ok(valid) => {
                if valid {
                    return handle_successful_auth(form_data["user"].clone());
                } else {
                    resp = "oh noes".to_string();
                }
            }
            Err(_) => {
                resp = LOGIN_HTML.to_string();
                println!("Unable to verify password hash")
            }
        },
        None => resp = "ERR".to_string(),
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(resp)
}

fn handle_successful_auth(user: String) ->  HttpResponse {
    let authCode = create_auth_code();
    add_auth_code(authCode.key.clone(), authCode.secret_bytes, user);
    // TODO this should actually redirect to the callback url with the code in it
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Auth Code: {}:{}", authCode.key , authCode.secret))
}

fn parse_form_data(body: Bytes) -> HashMap<String, String> {
    let mut form_data: HashMap<String, String> = HashMap::new();
    for (key, value) in form_urlencoded::parse(body.as_ref()) {
        form_data.insert(key.into_owned(), value.into_owned());
    }
    form_data
}
