use actix_session::Session;
use actix_web::{error::{ErrorInternalServerError, ErrorNotFound}, post, web, HttpRequest, HttpResponse, Responder, ResponseError, Result};
use actix_web::error::ErrorUnauthorized;
use openapi::models::login_user_payload::LoginUserPayload;
use redis::Commands;

use crate::AppContext;
use crate::auth::{AccessToken, RefreshToken};
use crate::user::User;

#[post("/login")]
pub async fn login(ctx: web::Data<AppContext>, payload: web::Json<LoginUserPayload>, session: Session) -> Result<impl Responder> {
    let login_data = payload.into_inner();

    let user = match User::get_user_by_name(&ctx.pg_pool, login_data.username).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(ErrorNotFound("User does not exist.")),
        Err(_) => return Err(ErrorInternalServerError("Failed to get user from database")),
    };

    if !user.password_matches(&login_data.password) {
        return Err(ErrorUnauthorized("Wrong password!"));
    }

    let mut session_store = match ctx.redis_client.get_connection() {
        Ok(store) => store,
        Err(_) => return Err(ErrorInternalServerError("Could not get redis Connection")),
    };

    let key = RefreshToken::session_key(user.id);
    let refresh_token = serde_json::to_string(&RefreshToken::new(user.id))?;

    session_store.set::<String, String, String>(key, refresh_token).map_err(ErrorInternalServerError)?;

    session.insert(AccessToken::SESSION_KEY, AccessToken::new(user.id))?;

    Ok(HttpResponse::Ok()).into()
}