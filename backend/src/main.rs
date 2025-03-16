mod app;
mod auth;
mod config;
mod user;
mod misc;

use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{self, scope},
    App, HttpServer,
};
use app::AppContext;
use crate::config::AppConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    let config = AppConfig::from_env();
    let ctx = AppContext::from_config(&config).await;

    HttpServer::new(move || {
        let cors = if cfg!(debug_assertions) {
            Cors::permissive()
        } else {
            Cors::default()
        };
        let auth = auth::middleware::JwtAuth::new(config.auth.clone());

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(ctx.clone()))
            .service(
                scope("api").service(misc::routes::health).service(
                    scope("user")
                        .wrap(auth)
                        .service(user::routes::get_user_by_id)
                        .service(user::routes::create_user)
                        .service(user::routes::get_all_users)
                        .service(user::routes::delete_user),
                ),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
