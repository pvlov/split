use std::{
    fs::{self, OpenOptions},
    io::ErrorKind,
    path::Path,
};

use actix_session::SessionExt;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::{ErrorInternalServerError, ErrorUnauthorized},
    HttpMessage,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use redis::Commands;

use std::sync::LazyLock;

use crate::{
    app::{AppContext, AppSession},
    auth::entities::{AccessToken, RefreshToken},
};

static JWT_HEADER: LazyLock<Header> = LazyLock::new(|| Header::new(Algorithm::ES256));
static JWT_VALIDATION: LazyLock<Validation> = LazyLock::new(|| Validation::new(Algorithm::ES256));

macro_rules! reject {
    ($err:expr) => {
        return Box::pin(futures::future::ready(Err::<_, actix_web::Error>($err)))
    };
}

macro_rules! proceed {
    ($service:expr, $req:expr) => {
        let fut: <S as actix_web::dev::Service<ServiceRequest>>::Future = $service.call($req);
        return Box::pin(async move { Ok(fut.await?) });
    };
}

/// Middleware for automagically checking auth for requests using JWTs.
///
/// The Auth flow is as follows:
///
/// When the user logs in or registers, they are given a refresh token which is stored in redis and
/// an access token which is stored in the clients cookies. When a client requests any route that
/// is not in the `NO_AUTH_REQUIRED` list, this middleware will check for the access token in the
/// session. If the access token is not present, the request will be rejected. If the access token
/// is present and has not expired yet, the request will be allowed to proceed. If the access token
/// is present but has expired, the middleware will check for a valid refresh token in redis to see if
/// the session can be refreshed. If a non-expired refresh token is found, a new access token will be generated
/// and the request will be allowed to proceed. If no valid session is found, the request will be
/// rejected.
///
/// As a side effect, an [`AppSession`] will be attached to the request as an extension if a valid access
/// token is found. This session can then be extracted from the request in the handler, if needed.
///
pub(crate) struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddleware::new(service))
    }
}

pub(crate) struct JwtAuthMiddleware<S> {
    service: S,
    signing_key: EncodingKey,
    verifying_key: DecodingKey,
}

impl<S> JwtAuthMiddleware<S> {
    pub fn new(service: S) -> Self {
        // Should point at a file in a read-only volume!
        let private_key_path = std::env::var("JWT_PRIVATE_KEY_PATH").expect("JWT_PRIVATE_KEY_PATH must be set");
        let public_key_path = std::env::var("JWT_PUBLIC_KEY_PATH").expect("JWT_PUBLIC_KEY_PATH must be set");

        let private_key = fs::read(&private_key_path).expect("Unable to read private key file");
        let public_key = fs::read(&public_key_path).expect("Unable to read public key file");

        let encoding_key = EncodingKey::from_ec_pem(&private_key).expect("Unable to create encoding key from private key");
        let decoding_key = DecodingKey::from_ec_pem(&public_key).expect("Unable to create decoding key from public key");

        Self {
            service,
            signing_key: encoding_key,
            verifying_key: decoding_key,
        }
    }
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();

        let user_id = match session.get::<String>(AccessToken::SESSION_KEY) {
            Ok(Some(token)) => {
                match jsonwebtoken::decode::<AccessToken>(&token, &self.verifying_key, &JWT_VALIDATION) {
                    Ok(token) => {
                        log::debug!("Access token is valid for {}", req.path());

                        let user_id = token.claims.sub;

                        if !token.claims.has_expired() {
                            // Access token is a valid jwt and has not expired
                            // we'll attach the user id to the request here so the handler can get
                            // it later
                            req.extensions_mut().insert(AppSession::new(user_id));

                            proceed!(self.service, req);
                        }

                        user_id
                    }
                    Err(why) => {
                        log::error!("Failed to decode access token: {}", why);
                        reject!(ErrorUnauthorized("Access token is invalid or expired"));
                    }
                }
            }
            Ok(None) => {
                log::debug!("No access token found in session for {}", req.path());
                reject!(ErrorUnauthorized("No access token found in session"));
            }
            Err(why) => {
                log::error!("Failed to get access token from session: {}", why);
                // We have to reject any request where accessing the session fails
                reject!(ErrorInternalServerError("Failed to get access token from session"));
            }
        };

        // At this point the access token is either not present, invalid in some form, or expired
        // We now want to check for a session in redis. If there is a valid session present, we can
        // just regenrate the access token and proceed with the request.

        let session_store = match req.app_data::<AppContext>() {
            Some(ctx) => ctx.redis_client.clone(),
            None => reject!(ErrorInternalServerError("AppContext not found in request extensions")),
        };

        let mut redis_conn = match session_store.get_connection() {
            Ok(conn) => conn,
            Err(why) => {
                log::error!("Failed to get redis connection: {}", why);
                reject!(ErrorInternalServerError("Failed to get redis connection"));
            }
        };

        let key = RefreshToken::to_session_key(user_id);

        let is_session_valid = match redis_conn.get::<String, String>(key) {
            Ok(raw_token) => {
                let refresh_token: RefreshToken = match serde_json::from_str(&raw_token) {
                    Ok(token) => token,
                    Err(why) => {
                        log::debug!("Failed to parse session: {}", why);
                        reject!(ErrorInternalServerError("Failed to parse session"));
                    }
                };

                !refresh_token.has_expired()
            }
            Err(why) => {
                log::debug!("No refresh token found for user {} because of {}", user_id, why);
                reject!(ErrorUnauthorized("No refresh token found"));
            }
        };

        if is_session_valid {
            // Generate a new access token
            let access_token = AccessToken::new(user_id);

            // Regenerate the access token

            let token = match jsonwebtoken::encode(&JWT_HEADER, &access_token, &self.signing_key) {
                Ok(token) => token,
                Err(why) => {
                    log::warn!("Failed to encode access token: {}", why);
                    reject!(ErrorInternalServerError("Failed to encode access token"));
                }
            };

            match session.insert(AccessToken::SESSION_KEY, token) {
                Ok(()) => {}
                Err(why) => {
                    log::warn!("Failed to set access token in session: {}", why);
                    reject!(ErrorInternalServerError("Failed to set access token in session"));
                }
            }

            // we'll attach the user id to the request here so the handler can get it later
            req.extensions_mut().insert(AppSession::new(user_id));

            proceed!(self.service, req);
        }

        reject!(ErrorUnauthorized("User does not have any sessions"));
    }
}
