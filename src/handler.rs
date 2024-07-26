use actix_web::{get, web, Responder};
use uuid::Uuid;

use crate::AppState;

#[get("/health")]
async fn health() -> impl Responder {
    "UP"
}

#[get("/")]
async fn create_user(app_state: web::Data<AppState>) -> impl Responder {

    let _ = sqlx::query("SELECT 1 + 1 as sum").fetch_one(&app_state.pg_pool).await.expect("Failed query");

    "get users!"
}

#[get("/users/{uid}")]
async fn get_user(_app_state: web::Data<AppState>, _uid: web::Path<Uuid>) -> impl Responder {
    "get!"
}
