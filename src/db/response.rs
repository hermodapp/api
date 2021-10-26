use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct Response {
    pub id: Uuid,
    pub form_id: Uuid,
}

impl Debug for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Response")
            .field("id", &self.id)
            .field("form_id", &self.form_id)
            .finish()
    }
}

pub struct NewResponse {
    pub id: Uuid,
    pub form_id: Uuid,
}

impl NewResponse {
    pub fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            form_id: Uuid::new_v4(),
        }
    }

    pub async fn store(&self, pool: &PgPool) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "INSERT INTO response (id, form_id)
             VALUES ($1, $2)",
            self.id,
            self.form_id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
