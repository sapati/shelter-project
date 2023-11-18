use crate::api::response::error::Status;
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub status: Status,
    pub token: String,
}
