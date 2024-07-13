use actix_web::{get, web, Responder};
use uuid::Uuid;

use crate::AppState;

#[get("/users/")]
async fn create_user(_app_state: web::Data<AppState>) -> impl Responder {
    "get users!"
}

#[get("/users/{uid}")]
async fn get_user(_app_state: web::Data<AppState>, _uid: web::Path<Uuid>) -> impl Responder {
    "get!"
}
