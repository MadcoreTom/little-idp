use actix_web::{App, HttpServer, web};
mod auth_code;
mod auth_token;
mod authorize;
mod db;
mod spa;
mod well_known;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::init_db();
    // db::add_user("tom".to_string(), "test".to_string());

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/oauth")
                    .route("/authorize", web::get().to(authorize::login_form))
                    .route("/authorize", web::post().to(authorize::login_submit))
                    .route("/token", web::post().to(auth_token::get_token)),
            )
            .service(web::scope("/.well-known").route(
                "openid-configuration",
                web::get().to(well_known::get_oidc_configuration),
            ))
            .service(
                web::scope("/")
                    .route("index.html", web::get().to(spa::spa_response))
                    .default_service(web::to(spa::spa_response)),
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
