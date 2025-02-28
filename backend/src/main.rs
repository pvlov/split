mod app;
mod handler;
mod entities;
mod auth;

use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{self, scope},
    App, HttpServer,
};
use app::AppContext;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    let ctx = AppContext::new().await;

    HttpServer::new(move || {
        let cors = Cors::default();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(ctx.clone()))
            .service(
                scope("api")
                .service(handler::misc_handler::health)
                .service(
                    scope("user")
                        .wrap(auth::middleware::JwtAuth)
                        .service(handler::user_handler::get_user_by_id)
                        .service(handler::user_handler::create_user)
                        .service(handler::user_handler::get_all_users)
                        .service(handler::user_handler::delete_user),
                ),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
