use std::future::{ready, Ready};

use actix_web::{error::ErrorInternalServerError, App, FromRequest, HttpMessage};
use sqlx::{postgres::PgPoolOptions, PgPool};
use uuid::Uuid;
use crate::config::{AppConfig, PostgresConfig, RedisConfig};

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
    pub async fn from_config(config: &AppConfig) -> Self {
        let pg_pool = Self::init_postgres(&config.postgres).await;
        let redis_client = Self::init_redis(&config.redis).await;

        Self {
            pg_pool,
            redis_client,
        }
    }

    async fn init_postgres(config: &PostgresConfig) -> PgPool {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.url)
            .await
            .expect("Failed to connect to Postgres DB");

        sqlx::migrate!().run(&pool).await.expect("Failed to run DB Migrations");

        pool
    }

    async fn init_redis(config: &RedisConfig) -> redis::Client {
        redis::Client::open(config.url.clone()).expect("Failed to connect to Redis")
    }
}
