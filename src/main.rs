use actix_web::{web, App, HttpServer};
mod authorize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/oauth")
            .route("/authorize", web::get().to(authorize::login_form))
            .route("/authorize", web::post().to(authorize::login_submit))
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}