use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::{auth::AuthenticationError, db::User, handlers::ApplicationError};

use super::ApplicationResponse;


#[tracing::instrument(name = "handlers::list_forms", skip(pool, id))]
pub async fn list_forms(pool: web::Data<PgPool>, id: Identity) -> ApplicationResponse {
    if let Some(id) = id.identity() {
        let current_user: User = serde_json::from_str(&id).unwrap();
        let forms = sqlx::query!(
            r#"
            SELECT * FROM form
            WHERE account_id=$1"#,
            current_user.id,
        )
        .fetch_all(pool.as_ref())
        .await
        .map_err(|e| ApplicationError::UnexpectedError(anyhow::anyhow!(e)))?;
        Ok(HttpResponse::Ok().body(format!("{:?},", forms)))
    } else {
        Err(ApplicationError::AuthError(
            AuthenticationError::Unauthorized,
        ))
    }
}
