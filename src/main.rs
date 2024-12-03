mod handler;
mod model;
mod repository;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone)]
struct AppContext {
    pg_pool: PgPool,
}

impl AppContext {
    pub async fn new() -> Self {
        let pool = Self::init_db().await;

        Self { pg_pool: pool }
    }

    async fn init_db() -> PgPool {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to Postgres DB");

        sqlx::migrate!().run(&pool).await.expect("Failed to run DB Migrations");

        pool
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    let app_data = AppContext::new().await;

    HttpServer::new(move || {
        let cors = Cors::default();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(app_data.clone()))
            .service(handler::user_handler::health)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
