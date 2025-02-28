use std::sync::LazyLock;

use chrono::DateTime;
use chrono::Utc;
use openapi::models::CreateUserPayload;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::PgPool;
use uuid::Uuid;

use anyhow::{bail, Error};

use openapi::models::user::User as OpenApiUser;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub(crate) struct User {
    pub id: Uuid,
    pub profile_picture: String,
    pub username: String,
    pub hashed_password: Vec<u8>,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub(crate) async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(&id)
            .fetch_optional(pool)
            .await;

        match user {
            Ok(user) => Ok(user),
            Err(why) => {
                log::error!(
                    "Something went wrong while querying for user by id in User::get_user_by_id: {}",
                    why
                );

                bail!("Failed to get user by id")
            }
        }
    }

    pub(crate) async fn get_user_by_name(pool: &PgPool, name: String) -> Result<Option<User>, Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(&name)
            .fetch_optional(pool)
            .await;

        match user {
            Ok(user) => Ok(user),
            Err(_) => bail!("Failed to get user by name")
        }
    }

    pub(crate) async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users").fetch_all(pool).await;

        match users {
            Ok(users) => Ok(users),
            Err(_) => bail!("Failed to get all users")
        }
    }

    pub(crate) async fn create_user(pool: &PgPool, payload: CreateUserPayload) -> Result<User, Error> {
        let hashed = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)?;

        let user = sqlx::query_as::<_, User>("INSERT INTO users (username, hashed_password, description) VALUES ($1, $2, $3) RETURNING *")
            .bind(&payload.username)
            .bind(&hashed)
            .bind(&payload.description)
            .fetch_one(pool)
            .await;

        match user {
            Ok(user) => Ok(user),
            Err(_) => bail!("Failed to create user")
        }
    }

    pub(crate) async fn update_user(pool: &PgPool, user: User) -> Result<User, Error> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET username = $1, hashed_password = $2, description = $3, updated_at = $4 WHERE id = $5 RETURNING *",
        )
        .bind(&user.username)
        .bind(&user.hashed_password)
        .bind(&user.description)
        .bind(&user.updated_at)
        .bind(&user.id)
        .fetch_one(pool)
        .await;

        match user {
            Ok(user) => Ok(user),
            Err(_) => bail!("Failed to update user")
        }
    }

    pub(crate) async fn delete_user(pool: &PgPool, id: Uuid) -> Result<Option<()>, Error> {
        let deleted = sqlx::query("DELETE FROM users WHERE id = $1").bind(&id).execute(pool).await;

        match deleted {
            Ok(res) if res.rows_affected() > 0 => Ok(Some(())),
            Ok(_) => Ok(None),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(_) => bail!("Failed to delete user")
        }
    }

    pub(crate) fn password_matches(&self, password: &str) -> bool {
        bcrypt::verify(&self.hashed_password, password).unwrap_or(false)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Into<OpenApiUser> for User {
    fn into(self) -> OpenApiUser {
        OpenApiUser::new(self.id, self.username, self.description)
    }
}
