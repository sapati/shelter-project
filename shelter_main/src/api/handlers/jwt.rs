use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};

use crate::api::response::error::ErrorResponse;
use crate::api::response::TokenClaims;
use crate::state::ApplicationState;
use jsonwebtoken::{decode, DecodingKey, Validation};

pub async fn auth<B>(
    State(state): State<Arc<ApplicationState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            auth_value
                .strip_prefix("Bearer ")
                .map(|stripped| stripped.to_owned())
        });

    let token = token.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: "error".to_string(),
            message: "Missing bearer token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let secret = &state.settings.load().token_secret;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| {
        let json_error = ErrorResponse {
            status: "error".to_string(),
            message: "Invalid bearer token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?
    .claims;

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
