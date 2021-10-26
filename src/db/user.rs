use std::fmt::Debug;
use std::str::FromStr;

use anyhow::Context;
use argon2::{password_hash::SaltString, Algorithm, Argon2, Params, PasswordHasher, Version};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

/// Represents a user record in the database.
#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("user_id", &self.id)
            .field("username", &self.username)
            .field("email", &self.email.as_ref().unwrap())
            .finish()
    }
}

impl User {
    pub async fn change_password(
        &self,
        pool: &PgPool,
        new_password: &str,
    ) -> Result<(), anyhow::Error> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        // Match production parameters
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None)?,
        )
        .hash_password(new_password.as_bytes(), &salt)?
        .to_string();
        sqlx::query!(
            "UPDATE account
             SET password = $1
             WHERE id = $2",
            password_hash,
            self.id,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

/// Struct used to create a new user in the database, password is hashed in `store()`
pub struct NewUser {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

impl NewUser {
    /// Struct with unqiue defaults for each field
    pub fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            username: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string(),
            email: None,
        }
    }

    pub fn new(username: String, password: String, email: Option<String>) -> Self {
        Self {
            username: username.to_ascii_lowercase(),
            password,
            email,
            ..Self::default()
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
            "INSERT INTO account (id, username, password, email)
            VALUES ($1, $2, $3, $4)",
            self.id,
            self.username.to_lowercase(),
            password_hash,
            self.email,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

/// Returns a user from the database with the given `user_id`.
pub async fn get_user_by_id(user_id: String, db_pool: &PgPool) -> Result<User, anyhow::Error> {
    let user_id = Uuid::from_str(&user_id)?;
    let user = sqlx::query_as!(User, "SELECT * FROM account WHERE id=$1", user_id)
        .fetch_one(db_pool)
        .await
        .context(format!("Failed to fetch user with user_id {}", user_id))?;
    Ok(user)
}
