use actix_web::{http::header::ContentType, HttpRequest, HttpResponse};
use serde_json::json;

const BASE_URL: &str = "http://localhost:8000";
// pub const ISS: &str = BASE_URL;

// See https://openid.net/specs/openid-connect-discovery-1_0.html#WellKnownRegistry
// This doesn't include all the mandatory details yet
pub async fn get_oidc_configuration(req: HttpRequest) -> HttpResponse {
    let base_url = get_base_url(req);

    let json = json!({
        "issuer": base_url,
        "authorization_endpoint": format!("{}/oauth/authorize", BASE_URL), // this has to be accessable via the front-channel
        "token_endpoint": format!("{}/oauth/token", base_url),
        "claims_supported": [
            "sub", "iss", "name"
        ]
    });

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json.to_string())
}

pub fn get_base_url(req: HttpRequest) -> String {
    let url = req.full_url();
    let scheme = url.scheme();
    let host = url.host().map(|h| h.to_string()).unwrap_or("localhost".to_string());
    let port = url.port();
    let port_str =  port.map(|p| format!(":{}", p)).unwrap_or("".to_string());
    let base_url = format!("{}://{}{}", scheme, host, port_str);

    println!("Base URL: {}", base_url);

    base_url.to_string()
}