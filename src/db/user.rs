use std::fmt::Debug;

use argon2::password_hash::SaltString;
use argon2::Algorithm;
use argon2::Argon2;
use argon2::Params;
use argon2::PasswordHasher;
use argon2::Version;
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

/// Represents a user record in the database.
#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("user_id", &self.id)
            .field("username", &self.username)
            .finish()
    }
}

/// Struct used to create a new user in the database, password is hashed in `store()`
pub struct NewUser {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

impl NewUser {
    /// Struct with unqiue defaults for each field
    pub fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            username: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string(),
        }
    }

    /// Store this struct in the users table
    pub async fn store(&self, pool: &PgPool) -> Result<(), anyhow::Error> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        // Match production parameters
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None)?,
        )
        .hash_password(self.password.as_bytes(), &salt)?
        .to_string();
        sqlx::query!(
            "INSERT INTO account (id, username, password)
            VALUES ($1, $2, $3)",
            self.id,
            self.username,
            password_hash,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
