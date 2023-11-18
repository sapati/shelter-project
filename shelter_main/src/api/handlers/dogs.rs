use crate::api::middleware::json::CustomJson;
use crate::api::response::dogs::DogCreateResponse;
use crate::api::response::error::{AppError, Status};
use crate::api::response::TokenClaims;
use crate::state::ApplicationState;
use axum::extract::State;
use axum::{debug_handler, Extension, Json};
use entity::dog::DogCreateRequest;
use sea_orm::{ActiveModelTrait, IntoActiveModel, TryIntoModel};
use std::sync::Arc;

#[debug_handler]
pub async fn create(
    Extension(_claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    CustomJson(payload): CustomJson<DogCreateRequest>,
) -> Result<Json<DogCreateResponse>, AppError> {
    let dog_active_model = payload.into_active_model();
    let dog_model = dog_active_model.save(state.db_conn.load().as_ref()).await?;
    let dog = dog_model.try_into_model()?;

    let response = DogCreateResponse {
        status: Status::Success,
        data: Some(dog),
    };

    Ok(Json(response))
}
