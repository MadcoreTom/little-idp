use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, Responder};

static SPA_HTML: &str = include_str!("spa.html");

pub async fn spa_response() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(SPA_HTML)
}
