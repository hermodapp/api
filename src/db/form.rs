use std::fmt::Debug;

use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct Form {
    pub id: Uuid,
    pub qr_code_id: Uuid,
    pub account_id: Uuid,
}

impl Debug for Form {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Form")
            .field("id", &self.id)
            .field("qr_code_id", &self.qr_code_id)
            .field("account_id", &self.account_id)
            .finish()
    }
}

pub struct NewForm {
    pub id: Uuid,
    pub qr_code_id: Uuid,
    pub account_id: Uuid,
}

impl NewForm {
    pub fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            qr_code_id: Uuid::new_v4(),
            account_id: Uuid::new_v4(),
        }
    }

    pub async fn store(&self, pool: &PgPool) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "INSERT INTO form (id, qr_code_id, account_id)
             VALUES ($1, $2, $3)",
            self.id,
            self.qr_code_id,
            self.account_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
