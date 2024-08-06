
use actix_session::Session;
use actix_web::{body::BoxBody, get, web, HttpResponse, Responder};
use log::{error, info};
use openapi::models::{ErrorResponse, UserLoginRequest, UserRegisterRequest};
use sqlx::Row;

use crate::{jwt_auth::JwtClaims, AppState};

#[get("/health")]
async fn health() -> impl Responder {
    "UP"
}

#[get("/user/register")]
async fn register_user(app_state: web::Data<AppState>, _session: Session, body: web::Json<UserRegisterRequest>) -> HttpResponse<BoxBody> {
    let is_conflict = sqlx::query("SELECT 1 FROM users WHERE username = $1")
        .bind(&body.name)
        .fetch_one(&app_state.pg_pool)
        .await;

    match is_conflict {
        Ok(row) => {
            if !row.is_empty() {
                return HttpResponse::BadRequest().body("Username or E-Mail already in use!");
            }
        }
        Err(why) => {
            error!(
                "Something went wrong while querying for username collision in handler::register_user: {}",
                why
            );

            return HttpResponse::InternalServerError().body("Something went wrong.");
        }
    }

    let password_hash = bcrypt::hash(&body.password, bcrypt::DEFAULT_COST).expect("bcrypt failed to encrypt password");

    let created = sqlx::query("INSERT INTO users (username, hashed_password, payment_description) VALUES ($1, $2, $3) RETURNING id")
        .bind(&body.name)
        .bind(&password_hash)
        .bind(&body.payment_description)
        .fetch_one(&app_state.pg_pool)
        .await
        .map(|id_row| id_row.get::<String, _>(0));

    match created {
        Ok(id) => HttpResponse::Created().body(id),
        Err(why) => {
            error!("Something went wrong when inserting new User in handler::create_user: {}", why);

            HttpResponse::InternalServerError().body("Something went wrong.")
        }
    }
}

#[get("/user/login")]
async fn login_user(app_state: web::Data<AppState>, session: Session, body: web::Json<UserLoginRequest>) -> HttpResponse<BoxBody> {
    // If there already is a JWT-Token set, that is valid and has not yet expired, the user does
    // not need to be logged in again
    if JwtClaims::try_from(&session).is_ok() {
        return HttpResponse::Ok().body("You are already succesfully logged in!");
    }

    let row = sqlx::query_as::<_, (String, String)>("SELCT (id, hashed_password) FROM users WHERE username = $1")
        .bind(&body.name)
        .fetch_one(&app_state.pg_pool)
        .await;


    match row {
        Ok((id, hash)) => {
            if bcrypt::verify(&body.password, &hash).is_err() {
                return HttpResponse::Unauthorized().body("Wrong password!");
            }

            let token = JwtClaims::new(id).encoded().expect("Failed to encode JwtToken");
            let _ = session.insert("jwt", token);
            HttpResponse::Ok().body("You have successfully logged in!")

        }
        Err(why) => match why {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().body("No such Account exists"),
            _ => HttpResponse::InternalServerError().body("Something went wrong"),
        },
    }
}

#[get("/user")]
async fn get_user(app_state: web::Data<AppState>, session: Session) -> HttpResponse<BoxBody> {
    match JwtClaims::try_from(&session) {
            Ok(token) => sqlx::query_as::<_, (String, String)>("SELECT (username, payment_description) FROM users WHERE id = $1")
            .bind(&token.id)
            .fetch_one(&app_state.pg_pool)
            .await
            .map(|_| HttpResponse::Ok().body("User!"))
            .unwrap_or(HttpResponse::InternalServerError().body("Something went wrong.")),
        Err(why) => {
            info!("Invalid AuthToken in session due to: {:?}", why);
            HttpResponse::Unauthorized().json(ErrorResponse::new(String::from("Something went wrong")))
            }
        }
}
