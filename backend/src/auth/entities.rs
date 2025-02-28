use serde::{Deserialize, Serialize};
use uuid::Uuid;

const ACCESS_TOKEN_LIFETIME: usize = 60 * 15; // 5min
const REFRESH_TOKEN_LIFETIME: usize = 60 * 60 * 24 * 30; // 30 days

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AccessToken {
    pub(crate) sub: Uuid,
    pub(crate) exp: usize,
}

impl AccessToken {
    pub const SESSION_KEY: &'static str = "jwt";

    #[inline(always)]
    pub fn has_expired(&self) -> bool {
        self.exp < chrono::Utc::now().timestamp() as usize
    }

    pub fn new(user_id: Uuid) -> Self {
        let now = chrono::Utc::now().timestamp() as usize;

        Self {
            sub: user_id,
            exp: now + ACCESS_TOKEN_LIFETIME,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RefreshToken {
    pub(crate) sub: Uuid,
    pub(crate) exp: usize,
}

impl RefreshToken {
    const PREFIX: &'static str = "refresh_token";

    #[inline(always)]
    pub fn has_expired(&self) -> bool {
        self.exp < chrono::Utc::now().timestamp() as usize
    }

    pub fn new(user_id: Uuid) -> Self {
        let now = chrono::Utc::now().timestamp() as usize;

        Self {
            sub: user_id,
            exp: now + REFRESH_TOKEN_LIFETIME,
        }
    }

    #[inline(always)]
    pub(crate) fn to_session_key(user_id: Uuid) -> String {
        format!("{}::{}", Self::PREFIX, user_id)
    }
}
