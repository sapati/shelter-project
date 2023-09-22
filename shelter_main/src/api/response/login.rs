use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub status: String,
    pub token: String,
}
