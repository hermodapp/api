use std::fmt::Debug;

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct ForgottenPasswordRequest {
    pub id: Uuid,
    pub account_id: Uuid,
    pub created_at: NaiveDateTime,
}

impl Debug for ForgottenPasswordRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Form")
            .field("id", &self.id)
            .field("created_at", &self.created_at)
            .field("account_id", &self.account_id)
            .finish()
    }
}

pub struct NewForgottenPasswordRequest {
    pub id: Uuid,
    pub account_id: Uuid,
    pub created_at: NaiveDateTime,
}

impl NewForgottenPasswordRequest {
    pub fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            account_id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
        }
    }

    pub fn new(account_id: Uuid) -> Self {
        Self {
            account_id,
            ..Self::default()
        }
    }

    pub async fn store(&self, pool: &PgPool) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "INSERT INTO forgotten_password_request (id, account_id, created_at)
             VALUES ($1, $2, $3)",
            self.id,
            self.account_id,
            self.created_at
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
