use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
