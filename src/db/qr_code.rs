use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct QrCode {
    pub id: Uuid,
    pub account_id: Uuid,
    pub slug: String,
    pub generation_data: String,
}