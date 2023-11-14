use serde::{Deserialize, Serialize};

pub mod dogs;
pub mod error;
pub mod login;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
