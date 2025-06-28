use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::http::header::ContentType;
use bytes::Bytes;
use url::form_urlencoded;
use std::collections::HashMap;

static LOGIN_HTML : &str = include_str!("login.html");

async fn login_form() -> impl Responder {
    HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(LOGIN_HTML)
}

async fn login_submit(body: Bytes) -> impl Responder {
    // Parse formdata body into map 
    let mut form_data: HashMap<String, String> = HashMap::new();
    for (key, value) in form_urlencoded::parse(body.as_ref()) {
        form_data.insert(key.into_owned(), value.into_owned());
    }

    // So in here we verify the username and password
    // Then make a time-limited token
    // and include it in a redirect

    HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(form_data["user"].clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/oauth")
            .route("/authorize", web::get().to(login_form))
            .route("/authorize", web::post().to(login_submit))
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}