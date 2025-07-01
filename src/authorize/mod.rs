use actix_web::{ Responder, HttpResponse};
use actix_web::http::header::ContentType;
use bytes::Bytes;
use url::form_urlencoded;
use std::collections::HashMap;

use crate::db;

static LOGIN_HTML : &str = include_str!("login.html");

pub async fn login_form() -> impl Responder {
    HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(LOGIN_HTML)
}

pub async fn login_submit(body: Bytes) -> impl Responder {
    // Parse formdata body into map 
    let form_data:HashMap<String, String>  = parse_form_data(body);

    // So in here we verify the username and password
    // Then make a time-limited token
    // and include it in a redirect
    let pass = db::get_user_pass(form_data["user"].clone());

    let resp: bool;
    match pass {
        Some(hash) => {
            match bcrypt::verify(&form_data["pass"], &hash) {
                Ok(valid) => resp = valid,
                Err(_) => {resp = false; println!("Unable to verify password hash") }
            }
        },
        None => resp = false
    };

    HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(if (resp) {"ok"}else{ "not ok"})
}

fn parse_form_data(body: Bytes) -> HashMap<String, String> {
    let mut form_data: HashMap<String, String> = HashMap::new();
    for (key, value) in form_urlencoded::parse(body.as_ref()) {
        form_data.insert(key.into_owned(), value.into_owned());
    }
    form_data
}