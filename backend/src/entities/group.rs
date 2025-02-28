use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use anyhow::{bail, Error};
use sqlx::PgPool;

use super::user::User;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub(crate) struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Group {
    pub(crate) async fn get_all_user_of_group(pool: &PgPool, group_id: Uuid) -> Result<Vec<User>, Error> {
        let users =
            sqlx::query_as::<_, User>("SELECT * FROM users u inner join group_membership gm ON u.id = gm.user_id WHERE gm.group_id = $1")
                .bind(&group_id)
                .fetch_all(pool)
                .await;

        match users {
            Ok(users) => Ok(users),
            Err(why) => {
                log::error!(
                    "Something went wrong while querying for all users of group in group_repository::get_all_user_of_group: {}",
                    why
                );

                bail!("Failed to get all users of group")
            }
        }
    }
}
