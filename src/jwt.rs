use crate::{
    auth::AuthenticationError,
    db::{get_user_by_id, User},
    handlers::ApplicationError,
};
use actix_web::HttpRequest;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::PgPool;
use uuid::Uuid;

/// Claims represents the JWT payload.
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
const KEY: &[u8] = b"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9";

pub fn encode_token(user_id: Uuid) -> Result<String, AuthenticationError> {
    Ok(encode_token_with_exp(user_id, 60 * 60)?)
}

pub fn encode_token_with_exp(user_id: Uuid, exp_offset: i64) -> Result<String, anyhow::Error> {
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
        &EncodingKey::from_secret(KEY),
    )?)
}

pub fn decode_token(token: &str) -> Result<Claims, anyhow::Error> {
    Ok(decode::<Claims>(
        token,
        &DecodingKey::from_secret(KEY),
        &Validation::default(),
    )?
    .claims)
}

pub async fn user_or_403(request: HttpRequest, pool: &PgPool) -> Result<User, ApplicationError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .ok_or_else(|| AuthenticationError::Unauthorized)?;
    let token = auth_header
        .to_str()
        .map_err(|e| AuthenticationError::UnexpectedError(anyhow::anyhow!(e)))?;
    let claims = decode_token(token)
        .map_err(|e| AuthenticationError::UnexpectedError(anyhow::anyhow!(e)))?;
    let user = get_user_by_id(claims.sub, pool).await?;
    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_and_decode_token() {
        let user_id = Uuid::new_v4();
        let token = encode_token(user_id).unwrap();
        let result = decode_token(&token).unwrap();
        assert_eq!(result.sub, user_id.to_string());
    }
}
