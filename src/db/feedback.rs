use std::fmt::Debug;

use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct Feedback {
    pub id: Uuid,
    pub form_input_id: Uuid,
    pub payload: String,
}

impl Debug for Feedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Feedback")
            .field("id", &self.id)
            .field("form_input_id", &self.form_input_id)
            .field("payload", &self.payload)
            .finish()
    }
}

pub struct NewFeedback {
    pub id: Uuid,
    pub form_input_id: Uuid,
    pub payload: String,
}

impl NewFeedback {
    pub fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            form_input_id: Uuid::new_v4(),
            payload: String::from(""),
        }
    }

    pub async fn store(&self, pool: &PgPool) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "INSERT INTO feedback (id, form_input_id, payload)
             VALUES ($1, $2, $3)",
            self.id,
            self.form_input_id,
            self.payload,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
