use actix_web::http::header::{ ContentType};
use actix_web::http::Uri;
use actix_web::{HttpRequest, HttpResponse, Responder};
use url::form_urlencoded;

use crate::db::find_auth_code;

pub async fn get_token(req: HttpRequest) -> impl Responder {
    println!("TOKEN {}", req.uri());

    let code = get_query_param(req.uri(), "code").unwrap_or_else(|| "".to_string());

    let x = find_auth_code(code);
    // TODO make a valid token (or should the user object include this info?)
    // TODO could use serde for this

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("{\"key\":\"example\"}")
}

fn get_query_param(uri: &Uri, key: &str) -> Option<String> {
    uri.query().and_then(|query| {
        form_urlencoded::parse(query.as_bytes())
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.into_owned())
    })
}
