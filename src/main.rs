mod handler;
mod ratelimiter;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

const PG_HOST: &str = "postgres";

#[derive(Clone)]
struct AppState {
    pg_pool: PgPool,
}

impl AppState {
    pub async fn new() -> Self {
        let pool = Self::init_db().await;

        Self { pg_pool: pool }
    }

    async fn init_db() -> PgPool {
        let connection_opts = PgConnectOptions::new().host(PG_HOST);

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(connection_opts)
            .await
            .expect("Failed to connect to Postgres DB");

        sqlx::migrate!().run(&pool).await.expect("Failed to run DB Migrations");

        pool
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let app_data = AppState::new().await;

    HttpServer::new(move || {
        let cors = Cors::default();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(app_data.clone()))
            .service(handler::create_user)
            .service(handler::health)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
