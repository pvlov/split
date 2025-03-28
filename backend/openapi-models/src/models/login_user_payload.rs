/*
 * Split
 *
 * An easy way to split expenses
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// LoginUserPayload : The object required to login a User
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginUserPayload {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl LoginUserPayload {
    /// The object required to login a User
    pub fn new(username: String, password: String) -> LoginUserPayload {
        LoginUserPayload {
            username,
            password,
        }
    }
}

