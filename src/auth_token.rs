use actix_web::http::Uri;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde_json::json;
use url::form_urlencoded;

use crate::db::{find_auth_code, get_user};
use crate::well_known::ISS;

pub async fn get_token(req: HttpRequest) -> impl Responder {
    println!("TOKEN {}", req.uri());

    let code = get_query_param(req.uri(), "code").unwrap_or_else(|| "".to_string());

    let json = match find_auth_code(code) {
        Some(user) => get_user(user).map(|data| {
            json!({
                "sub": data.sub,
                "iss": ISS,
                "name": data.name
            })
        }),
        None => {
            println!("User not found");
            None
        }
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(
        json.map(|j| j.to_string())
            .unwrap_or_else(|| "ERR".to_string()),
    )
}

fn get_query_param(uri: &Uri, key: &str) -> Option<String> {
    uri.query().and_then(|query| {
        form_urlencoded::parse(query.as_bytes())
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.into_owned())
    })
}
