use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct Form {
    pub id: Uuid,
    pub account_id: Uuid,
    pub title: String,
}

impl Debug for Form {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Form")
            .field("id", &self.id)
            .field("title", &self.title)
            .field("account_id", &self.account_id)
            .finish()
    }
}

pub struct NewForm {
    pub id: Uuid,
    pub account_id: Uuid,
    pub title: String,
}

impl NewForm {
    pub fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: String::new(),
            account_id: Uuid::new_v4(),
        }
    }

    pub async fn store(&self, pool: &PgPool) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "INSERT INTO form (id, account_id, title)
             VALUES ($1, $2, $3)",
            self.id,
            self.account_id,
            self.title
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
