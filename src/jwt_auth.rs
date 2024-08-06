use actix_session::Session;
use jiff::{tz::TimeZone, ToSpan, Zoned};
use jsonwebtoken::{errors::Error, EncodingKey, Header};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JwtClaims {
    pub id: String,
    pub exp: Zoned,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum JwtError {
    NoToken,
    Expired,
    SigCheckFailed,
}

impl TryFrom<&Session> for JwtClaims {
    type Error = JwtError;

    fn try_from(_session: &Session) -> Result<Self, Self::Error> {
        Err(JwtError::NoToken)
    }
}

impl JwtClaims {
    pub fn new(uid: String) -> Self {
        let expire = Zoned::now().with_time_zone(TimeZone::UTC).saturating_add(30.days());

        Self { id: uid, exp: expire }
    }

    pub fn encoded(&self) -> Result<String, Error> {
        jsonwebtoken::encode(&Header::default(), self, &EncodingKey::from_secret("secret".as_ref()))
    }
}
