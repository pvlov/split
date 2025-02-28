
use actix_session::Session;
use actix_web::{error::{ErrorInternalServerError, ErrorNotFound}, post, web, HttpResponse, Responder};
use openapi::models::login_user_payload::LoginUserPayload;
use redis::Commands;

use crate::{auth::entities::{AccessToken, RefreshToken}, entities::user::User, AppContext};

const REDIS_OK: &str = "OK";

#[post("/login")]
pub async fn login(ctx: web::Data<AppContext>, payload: web::Json<LoginUserPayload>, session: Session) -> impl Responder {
    let login_data = payload.into_inner();

    let user = match User::get_user_by_name(&ctx.pg_pool, login_data.username).await {
        Ok(Some(user)) => user,
        Ok(None) => return ErrorNotFound("User does not exist.").into(),
        Err(_) => return ErrorInternalServerError("Failed to get user from database").into(),
    };

    if !user.password_matches(&login_data.password) {
        return HttpResponse::Unauthorized().finish();
    }

    let mut session_store = match ctx.redis_client.get_connection() {
        Ok(store) => store,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let key = RefreshToken::to_session_key(user.id);

    let refresh_token = match serde_json::to_string(&RefreshToken::new(user.id)) {
        Ok(token) => token,
        Err(why) => {
            log::error!("Failed to serialize RefreshToken due to {}", why);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match session_store.set::<String, String, String>(key, refresh_token){
        Ok(ret) if ret != REDIS_OK => {
            log::error!("Expected redis to return {} on success, but got {}", REDIS_OK, ret);
            return HttpResponse::InternalServerError().finish();
        }
        Err(why) => {
            log::error!("Could not insert refresh token into session store due to: {}", why);
            return HttpResponse::InternalServerError().finish();
        }
        _ => ()
    }

    match session.insert(AccessToken::SESSION_KEY, AccessToken::new(user.id)) {
        Ok(()) => (),
        Err(why) => {
            log::error!("Could not insert access token into cookie session due to: {}", why);
            return HttpResponse::InternalServerError().finish();
        }
    }

    HttpResponse::Ok().finish()
}
