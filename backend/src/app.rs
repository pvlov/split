use std::future::{ready, Ready};

use actix_web::{error::ErrorInternalServerError, FromRequest, HttpMessage};
use sqlx::{postgres::PgPoolOptions, PgPool};
use uuid::Uuid;

// While i kind of hate this, the key for any value you attach to a request is type-based.
#[derive(Default, Clone)]
pub struct AppSession {
    pub(crate) user_id: Uuid,
}

impl AppSession {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

impl FromRequest for AppSession {
    type Error = actix_web::Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req.extensions().get::<AppSession>() {
            Some(app_session) => ready(Ok(app_session.clone())),
            None => ready(Err(ErrorInternalServerError(
                "AppSession not found in request extensions, did you attach the jwt middleware?",
            ))),
        }
    }
}


#[derive(Clone)]
pub struct AppContext {
    pub(crate) pg_pool: PgPool,
    pub(crate) redis_client: redis::Client,
}

impl AppContext {
    pub async fn new() -> Self {
        let pg_pool = Self::init_db().await;
        let session_store = Self::init_redis().await;

        Self { pg_pool, redis_client: session_store }
    }

    async fn init_db() -> PgPool {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let max_connections = std::env::var("MAX_CONNECTIONS")
            .expect("MAX_CONNECTIONS must be set")
            .parse::<u32>()
            .expect("MAX_CONNECTIONS must be a number");

        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&database_url)
            .await
            .expect("Failed to connect to Postgres DB");

        sqlx::migrate!().run(&pool).await.expect("Failed to run DB Migrations");

        pool
    }

    async fn init_redis() -> redis::Client {
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

        redis::Client::open(redis_url).expect("Failed to connect to Redis")
    }
}

