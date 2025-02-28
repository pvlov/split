use actix_web::{
    delete, get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use openapi::models::create_user_payload::CreateUserPayload;
use uuid::Uuid;

use crate::{entities::user::User, AppContext};

// Important: any of these handlers are prefixed with "user"

#[get("/{id}")]
async fn get_user_by_id(ctx: web::Data<AppContext>, id: web::Path<Uuid>) -> impl Responder {
    let id = id.into_inner();

    let user = User::get_user_by_id(&ctx.pg_pool, id).await;

    match user {
        Ok(Some(user)) => {
            let user = serde_json::to_string(&user).expect("Failed to serialize User");
            HttpResponse::Ok().body(user)
        }
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(why) => {
            log::error!(
                "Something went wrong when querying for user by id in handler::get_user_by_id: {}",
                why
            );
            HttpResponse::InternalServerError().body("Something went wrong")
        }
    }
}

#[post("/")]
async fn create_user(ctx: web::Data<AppContext>, body: Json<CreateUserPayload>) -> impl Responder {
    let payload = body.into_inner();

    let user = User::create_user(&ctx.pg_pool, payload).await;

    match user {
        Ok(user) => {
            let user = serde_json::to_string(&user).expect("Failed to serialize User");
            HttpResponse::Created().body(user)
        }
        Err(why) => {
            log::error!("Something went wrong when creating a user in handler::create_user: {}", why);
            HttpResponse::InternalServerError().body("Something went wrong")
        }
    }
}

#[get("/")]
async fn get_all_users(ctx: web::Data<AppContext>) -> impl Responder {
    let users = User::get_all_users(&ctx.pg_pool).await;

    match users {
        Ok(users) => {
            let users = serde_json::to_string(&users).expect("Failed to serialize Users");
            HttpResponse::Ok().body(users)
        }
        Err(why) => {
            log::error!(
                "Something went wrong when querying for all users in handler::get_all_users: {}",
                why
            );
            HttpResponse::InternalServerError().body("Something went wrong")
        }
    }
}

#[delete("/{id}")]
async fn delete_user(ctx: web::Data<AppContext>, id: web::Path<Uuid>) -> impl Responder {
    let id = id.into_inner();

    let user = User::delete_user(&ctx.pg_pool, id).await;

    match user {
        Ok(Some(())) => HttpResponse::Ok().body("User deleted"),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(why) => {
            log::error!("Something went wrong when deleting a user in handler::delete_user: {}", why);
            HttpResponse::InternalServerError().body("Something went wrong")
        }
    }
}

#[post("/{id}")]
async fn update_user(ctx: web::Data<AppContext>, payload: Json<String>) -> impl Responder {
    HttpResponse::Ok().body("update_user")
}

//
// #[get("/user/register")]
// async fn register_user(app_state: web::Data<AppState>, body: web::Json<UserRegisterRequest>) -> impl Responder {
//     let is_conflict = sqlx::query("SELECT 1 FROM users WHERE username = $1")
//         .bind(&body.name)
//         .fetch_one(&app_state.pg_pool)
//         .await;
//
//     match is_conflict {
//         Ok(row) => {
//             if !row.is_empty() {
//                 return HttpResponse::BadRequest().body("Username already in use!");
//             }
//         }
//         Err(why) => {
//             log::error!(
//                 "Something went wrong while querying for username collision in handler::register_user: {}",
//                 why
//             );
//
//             return HttpResponse::InternalServerError().body(DEFAULT_INTERNAL_SERVER_ERROR_MESSAGE);
//         }
//     }
//
//     let password_hash = bcrypt::hash(&body.password, bcrypt::DEFAULT_COST).expect("bcrypt failed to encrypt password");
//
//     let created = sqlx::query_as::<_, (String,)>("INSERT INTO users (username, hashed_password, description) VALUES ($1, $2, $3) RETURNING id")
//         .bind(&body.name)
//         .bind(&password_hash)
//         .bind(&body.description)
//         .fetch_one(&app_state.pg_pool)
//         .await
//         .map(|id_row| id_row.0);
//
//
//
//     match created {
//         Ok(id) => HttpResponse::Created().body(id),
//         Err(why) => {
//             log::error!("Something went wrong when inserting new User in handler::create_user: {}", why);
//
//             HttpResponse::InternalServerError().body(DEFAULT_INTERNAL_SERVER_ERROR_MESSAGE)
//         }
//     }
// }
//
//
//
// #[get("/user/login")]
// async fn login_user(app_state: web::Data<AppState>, session: Session, body: web::Json<UserLoginRequest>) -> HttpResponse<BoxBody> {
//     let row = sqlx::query_as::<_, (String, String)>("SELECT (id, hashed_password) FROM users WHERE username = $1")
//         .bind(&body.name)
//         .fetch_one(&app_state.pg_pool)
//         .await;
//
//
//     match row {
//         Ok((id, hash)) => {
//             if bcrypt::verify(&body.password, &hash).is_err() {
//                 return HttpResponse::Unauthorized().body("Wrong password!");
//             }
//
//             let token = JwtClaims::new(id).encoded().expect("Failed to encode JwtToken");
//             if let Err(why) = session.insert("jwt", token) {
//                 log::error!("Something went wrong when inserting jwt token into session: {}", why);
//                 return HttpResponse::InternalServerError().body("Something went wrong");
//             }
//             HttpResponse::Ok().body("You are succesfully logged in!")
//         }
//         Err(why) => match why {
//             sqlx::Error::RowNotFound => HttpResponse::NotFound().body("No such Account exists"),
//             _ => HttpResponse::InternalServerError().body("Something went wrong"),
//         },
//     }
// }
//
// #[get("/user")]
// async fn get_user(app_state: web::Data<AppState>, session: Session) -> HttpResponse<BoxBody> {
//     match JwtClaims::try_from(&session) {
//             Ok(token) => sqlx::query_as::<_, (String, String)>("SELECT (username, description) FROM users WHERE id = $1")
//             .bind(&token.id)
//             .fetch_one(&app_state.pg_pool)
//             .await
//             .map(|_| HttpResponse::Ok().body("User!"))
//             .unwrap_or(HttpResponse::InternalServerError().body("Something went wrong.")),
//         Err(why) => {
//             log::info!("Invalid AuthToken in session due to: {:?}", why);
//             HttpResponse::Unauthorized().json(ErrorResponse::new(String::from("Something went wrong")))
//             }
//         }
// }
