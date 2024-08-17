use actix_session::Session;
use jiff::{tz::TimeZone, ToSpan, Zoned};
use jsonwebtoken::{decode, errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

    fn try_from(session: &Session) -> Result<Self, Self::Error> {
        let token = session
            .get::<String>("jwt")
            .map_err(|_| JwtError::NoToken)?
            .ok_or(JwtError::NoToken)?;

        let decoding_key = DecodingKey::from_secret("secret".as_ref());
        let validation = Validation::default();

        match decode::<JwtClaims>(&token, &decoding_key, &validation) {
            Ok(token_data) => {
                // Check if the token is expired
                if Zoned::now().with_time_zone(TimeZone::UTC) >= token_data.claims.exp {
                    Err(JwtError::Expired)
                } else {
                    Ok(token_data.claims)
                }
            }
            Err(_) => Err(JwtError::SigCheckFailed),
        }
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
