use actix_web::{HttpResponse, http::header::ContentType};
use serde_json::json;

// See https://openid.net/specs/openid-connect-discovery-1_0.html#WellKnownRegistry
// This doesn't include all the mandatory details yet
pub async fn get_oidc_configuration() -> HttpResponse {
    let base_url = "http://localhost:8080";
    let json = json!({
        "issuer": base_url,
        "authorization_endpoint": format!("{}/oauth/authorize", base_url),
        "token_endpoint": format!("{}/oauth/token", base_url),
        "claims_supported": [
            "sub", "iss", "name"
        ]
    });

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json.to_string())
}
