use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct Field {
    pub id: Uuid,
    pub form_id: Uuid,
    pub field_type: String,
    pub caption: String,
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field")
            .field("id", &self.id)
            .field("form_id", &self.form_id)
            .field("field_type", &self.field_type)
            .field("caption", &self.caption)
            .finish()
    }
}

pub struct NewField {
    pub id: Uuid,
    pub form_id: Uuid,
    pub field_type: String,
    pub caption: String,
}

impl NewField {
    pub fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            form_id: Uuid::new_v4(),
            field_type: String::new(),
            caption: String::new(),
        }
    }

    pub async fn store(&self, pool: &PgPool) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "INSERT INTO form_input (id, form_id, type, caption)
             VALUES ($1, $2, $3, $4)",
            self.id,
            self.form_id,
            self.field_type,
            self.caption,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
