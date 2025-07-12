use std::collections::HashMap;

use actix_web::http::Uri;
use actix_web::http::header::ContentType;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};
use base64::prelude::{Engine, BASE64_STANDARD_NO_PAD};
use bytes::Bytes;
use serde_json::json;
use url::form_urlencoded;

use crate::db::{get_user_by_auth_code};
use crate::well_known::get_base_url;

pub async fn get_token(req: HttpRequest, body: Bytes) -> impl Responder {
    println!("TOKEN {}", req.uri());
    let form_data: HashMap<String, String> = parse_form_data(body);
    // TODO expect a form body lie https://docs.aws.amazon.com/cognito/latest/developerguide/token-endpoint.html
    let code = form_data.get("code").map(|c| c.to_string()).unwrap_or("".to_string());

    let json = get_user_by_auth_code(code.as_str()).map(|data| {
        data.user.map(|user| json!({
          "sub": user.sub,
          "iss": get_base_url(req),
          "name": user.name,
          "aud": form_data.get("client_id"),
          "nonce": data.code.nonce
        })
    )
    }).unwrap_or( None);

    println!("{} => {}", code, json.clone().map(|j| j.to_string()).unwrap_or("ERR".to_string()));

    let jwt = to_unsigned_jwt(
         json.map(|j| j.to_string()).unwrap_or_else(|| "ERR".to_string())
        );

    println!("JWT: {}", jwt);

    let payload = json!({
        "token_type": "bearer",
        "expires_in": 3600,
        "id_token": jwt,
        "access_token": jwt
    }).to_string();

    println!("payload: {}", payload);

    HttpResponse::Ok().content_type(ContentType::json()).body(payload)
}

fn get_query_param(uri: &Uri, key: &str) -> Option<String> {
    uri.query().and_then(|query| {
        form_urlencoded::parse(query.as_bytes())
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.into_owned())
    })
}

// TODO duplicated code. move to a util
fn parse_form_data(body: Bytes) -> HashMap<String, String> {
    let mut form_data: HashMap<String, String> = HashMap::new();
    for (key, value) in form_urlencoded::parse(body.as_ref()) {
        form_data.insert(key.into_owned(), value.into_owned());
    }
    form_data
}

fn to_unsigned_jwt(payload:String) -> String{
    let header = "{\"alg\": \"none\",\"typ\":\"JWT\"}";
    let header_b64 = BASE64_STANDARD_NO_PAD.encode(header);
    let payload_b64 = BASE64_STANDARD_NO_PAD.encode(payload);
    format!("{}.{}.", header_b64, payload_b64)
}