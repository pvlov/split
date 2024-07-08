mod user_handler;

use actix_web::{App, HttpServer};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

const PGHOST: &str = "postgres";

#[allow(dead_code)]
struct SplitApp {
    pg_pool: PgPool,
}

impl SplitApp {
    pub async fn new() -> Self {
        let pool = SplitApp::init_db().await;

        Self { pg_pool: pool }
    }

    async fn init_db() -> PgPool {
        let connection_opts = PgConnectOptions::new().host(PGHOST);

        PgPoolOptions::new()
            .max_connections(5)
            .connect_with(connection_opts)
            .await
            .expect("Failed to connect to Postgres DB")
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _app = SplitApp::new().await;
    
    HttpServer::new(|| App::new().service(user_handler::create_user).service(user_handler::get_user))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
