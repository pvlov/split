use actix_web::{delete, get, post, web, HttpResponse, Responder};
use actix_web::web::Json;
use openapi::models::CreateUserPayload;
use uuid::Uuid;
use crate::app::AppContext;
use crate::user::User;


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