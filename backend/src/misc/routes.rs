use actix_web::{get, Responder};

#[get("/health")]
pub(crate) async fn health() -> impl Responder {
    "UP"
}