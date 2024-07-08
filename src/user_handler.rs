use actix_web::{get, web, Responder};
use uuid::Uuid;

#[get("/users/")]
async fn create_user() -> impl Responder {
    "get!"
}

#[get("/users/{uid}")]
async fn get_user(_uid: web::Path<Uuid>) -> impl Responder {
    "get!" 
}
