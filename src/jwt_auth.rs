use actix_session::Session;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug)]
pub struct AuthToken {
    id: Uuid,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum AuthTokenError {
    NoToken,
    Expired,
    SigCheckFailed,
}

impl TryFrom<Session> for AuthToken {
    type Error = AuthTokenError;

    fn try_from(_session: Session) -> Result<Self, Self::Error> {
        Err(AuthTokenError::NoToken)
    }
}
