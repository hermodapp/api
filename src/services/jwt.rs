//! Contains code for serializing and deserializng JSON Web Tokens.
use crate::{
    db::{get_user_by_id, User},
    handlers::ApplicationError,
    services::auth::AuthenticationError,
};
use actix_web::HttpRequest;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::PgPool;
use uuid::Uuid;

/// Service to manage JWT tokens
pub struct JwtClient {
    auth_key: String,
    pool: PgPool,
}

impl JwtClient {
    /// Create a new JWT Client
    pub fn new(auth_key: String, pool: PgPool) -> Self {
        Self { auth_key, pool }
    }

    /// Encode a JWT token for the given user
    #[tracing::instrument(name = "services::jwt::encode_token", skip(self))]
    pub fn encode_token(&self, user_id: Uuid) -> Result<String, AuthenticationError> {
        Ok(self.encode_token_with_exp(user_id, 60 * 60)?)
    }

    /// Encode a JWT token with a custom expiration time
    #[tracing::instrument(name = "services::jwt::encode_token_with_exp", skip(self))]
    pub fn encode_token_with_exp(
        &self,
        user_id: Uuid,
        exp_offset: i64,
    ) -> Result<String, anyhow::Error> {
        let my_iat = Utc::now().timestamp();
        let my_exp = Utc::now()
            .checked_add_signed(Duration::seconds(exp_offset))
            .expect("invalid timestamp")
            .timestamp();

        let my_claims = Claims {
            sub: user_id.to_string(),
            iat: my_iat as usize,
            exp: my_exp as usize,
        };

        Ok(encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(self.auth_key.as_bytes()),
        )?)
    }

    /// Decode a JWT token and provide its claims if it is valid
    #[tracing::instrument(name = "services::jwt::decode_token", skip(self))]
    pub fn decode_token(&self, token: &str) -> Result<Claims, anyhow::Error> {
        Ok(decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.auth_key.as_bytes()),
            &Validation::default(),
        )?
        .claims)
    }

    /// Given an incoming HTTP request, return the user currently logged in. If there is no
    /// user logged in, generate a `403 Unauthorized` error response.
    #[tracing::instrument(name = "services::jwt::user_or_403", skip(self, request))]
    pub async fn user_or_403(&self, request: HttpRequest) -> Result<User, ApplicationError> {
        let auth_header = request
            .headers()
            .get("Authorization")
            .ok_or(AuthenticationError::Unauthorized)?;
        let token = auth_header
            .to_str()
            .map_err(|e| AuthenticationError::UnexpectedError(anyhow::anyhow!(e)))?;
        let claims = self
            .decode_token(token)
            .map_err(|e| AuthenticationError::UnexpectedError(anyhow::anyhow!(e)))?;
        let user = get_user_by_id(claims.sub, &self.pool).await?;
        tracing::Span::current().record("username", &tracing::field::display(&user.username));
        tracing::Span::current().record("user_id", &tracing::field::display(&user.id));
        Ok(user)
    }
}

/// Claims represents the JWT payload.
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
