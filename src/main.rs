use actix_web::{App, HttpServer, web};
mod auth_code;
mod authorize;
mod db;
mod spa;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::add_user("tom".to_string(), "test".to_string());

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/oauth")
                    .route("/authorize", web::get().to(authorize::login_form))
                    .route("/authorize", web::post().to(authorize::login_submit)),
            )
            .service(
                web::scope("/")
                    .route("index.html", web::get().to(spa::spa_response))
                    .default_service(web::to(spa::spa_response)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
