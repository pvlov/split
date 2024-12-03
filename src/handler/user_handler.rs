use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{model::User, repository::user_repository, AppContext};

#[get("/health")]
async fn health() -> impl Responder {
    "UP"
}

#[get("/user/{id}")]
async fn get_user_by_id(ctx: web::Data<AppContext>, id: web::Path<Uuid>) -> impl Responder {
    let id = id.into_inner();

    let user = user_repository::get_user_by_id(&ctx.pg_pool, id).await;

    match user {
        Ok(Some(user)) => {
            let user = serde_json::to_string(&user).expect("Failed to serialize User");
            HttpResponse::Ok().body(user)
        },
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


#[post("/user")]
async fn create_user(ctx: web::Data<AppContext>, user: web::Json<User>) -> impl Responder {
    let user = user.into_inner();

    let user = user_repository::create_user(&ctx.pg_pool, user).await;

    match user {
        Ok(user) => {
            let user = serde_json::to_string(&user).expect("Failed to serialize User");
            HttpResponse::Created().body(user)
        }
        Err(why) => {
            log::error!(
                "Something went wrong when creating a user in handler::create_user: {}",
                why
            );
            HttpResponse::InternalServerError().body("Something went wrong")
        }
    }
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

#[get("/user")]
async fn get_user(_: web::Data<AppContext>) -> impl Responder {
    ""
}
