use anyhow::{anyhow, Error};
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::User;

pub(crate) async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&id)
        .fetch_one(pool)
        .await;

    match user {
        Ok(user) => Ok(Some(user)),
        Err(sqlx::Error::RowNotFound) => Ok(None),
        Err(why) => {
            log::error!(
                "Something went wrong while querying for user by id in user_repository::get_user_by_id: {}",
                why
            );

            Err(anyhow!("Failed to get user by id"))
        }
    }
}

pub(crate) async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users").fetch_all(pool).await;

    match users {
        Ok(users) => Ok(users),
        Err(why) => {
            log::error!(
                "Something went wrong while querying for all users in user_repository::get_all_users: {}",
                why
            );

            Err(anyhow!("Failed to get all users"))
        }
    }
}

pub(crate) async fn create_user(pool: &PgPool, user: User) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, username, hashed_password, description, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(&user.id)
    .bind(&user.username)
    .bind(&user.hashed_password)
    .bind(&user.description)
    .bind(&user.created_at)
    .bind(&user.updated_at)
    .fetch_one(pool)
    .await;

    match user {
        Ok(user) => Ok(user),
        Err(why) => {
            log::error!(
                "Something went wrong while creating a user in user_repository::create_user: {}",
                why
            );

            Err(anyhow!("Failed to create user"))
        }
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
        Err(why) => {
            log::error!(
                "Something went wrong while updating a user in user_repository::update_user: {}",
                why
            );

            Err(anyhow!("Failed to update user"))
        }
    }
}

pub(crate) async fn delete_user(pool: &PgPool, id: Uuid) -> Result<(), Error> {
    let _ = sqlx::query("DELETE FROM users WHERE id = $1").bind(&id).execute(pool).await;

    Ok(())
}
