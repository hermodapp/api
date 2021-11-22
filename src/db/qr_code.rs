use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct QrCode {
    pub id: Uuid,
    pub account_id: Uuid,
    pub slug: String,
    pub generation_data: String,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub payload: Option<String>,
    pub form_id: Option<Uuid>,
}
