use actix_web::{HttpResponse, http::header::ContentType};
use serde_json::json;

const BASE_URL: &str = "http://localhost:8080";
pub const ISS: &str = BASE_URL;

// See https://openid.net/specs/openid-connect-discovery-1_0.html#WellKnownRegistry
// This doesn't include all the mandatory details yet
pub async fn get_oidc_configuration() -> HttpResponse {
    let json = json!({
        "issuer": ISS,
        "authorization_endpoint": format!("{}/oauth/authorize", BASE_URL),
        "token_endpoint": format!("{}/oauth/token", BASE_URL),
        "claims_supported": [
            "sub", "iss", "name"
        ]
    });

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json.to_string())
}
