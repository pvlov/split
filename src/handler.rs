use actix_web::{body::BoxBody, get, web, HttpResponse, Responder};
use openapi::models::UserRegisterRequest;
use sqlx::Row;

use crate::AppState;

#[get("/health")]
async fn health() -> impl Responder {
    "UP"
}

#[get("/user/register")]
async fn create_user(app_state: web::Data<AppState>, body: web::Json<UserRegisterRequest>) -> HttpResponse<BoxBody> {


    let is_conflict = sqlx::query("SELECT 1 FROM users WHERER username = $1")
        .bind(body.name.clone())
        .fetch_one(&app_state.pg_pool)
        .await;


    match is_conflict {
        Ok (row) => {
           if !row.is_empty() {
                return HttpResponse::Conflict().body("Username or E-Mail already in use!")
            }
        },
        Err (_) => {
            return HttpResponse::InternalServerError().body("Something went wrong.");
        }
    }


    return HttpResponse::Created().into();
}
