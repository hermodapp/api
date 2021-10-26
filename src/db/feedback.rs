use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct Feedback {
    pub id: Uuid,
    pub response_id: Uuid,
    pub form_input_id: Uuid,
    pub content: String,
}

impl Debug for Feedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Feedback")
            .field("id", &self.id)
            .field("form_input_id", &self.form_input_id)
            .field("content", &self.content)
            .field("response_id", &self.response_id)
            .finish()
    }
}

pub struct NewFeedback {
    pub id: Uuid,
    pub form_input_id: Uuid,
    pub content: String,
    pub response_id: Uuid,
}

impl NewFeedback {
    pub fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            form_input_id: Uuid::new_v4(),
            content: String::new(),
            response_id: Uuid::new_v4(),
        }
    }

    pub async fn store(&self, pool: &PgPool) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "INSERT INTO feedback (id, form_input_id, content, response_id)
             VALUES ($1, $2, $3, $4)",
            self.id,
            self.form_input_id,
            self.content,
            self.response_id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
