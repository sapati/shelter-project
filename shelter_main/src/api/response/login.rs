use crate::api::response::error::Status;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub status: Status,
    pub token: String,
}
