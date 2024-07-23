mod handler;
mod ratelimiter;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::{
    migrate::Migrator,
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

const PG_HOST: &str = "postgres";

static MIGRATOR: Migrator = sqlx::migrate!();

#[allow(dead_code)]
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

        MIGRATOR.run(&pool).await.expect("Failed to run DB Migrations");

        pool
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(async { web::Data::new(AppState::new().await) })
            .service(handler::create_user)
            .service(handler::get_user)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
