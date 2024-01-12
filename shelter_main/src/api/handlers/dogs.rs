use crate::api::middleware::json::CustomJson;
use crate::api::response::dogs::{DogCreateResponse, DogGetResponse, DogListResponse};
use crate::api::response::error::{AppError, Status};
use crate::api::response::TokenClaims;
use crate::state::ApplicationState;
use axum::extract::{Path, State};
use axum::{debug_handler, Extension, Json};
use entity::dog::DogCreateRequest;
use entity::dog::Entity as Dog;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, TryIntoModel};
use std::sync::Arc;
use tracing::instrument;

#[utoipa::path(
    post,
    path = "/dogs",
    tag = "dogs",
    request_body = DogCreateRequest,
    responses(
        (status = 200, description = "Dog create", body = DogCreateResponse),
        (status = 401, description = "Missing bearer token", body = ErrorResponse),
    ),
    security(
        ("api_jwt_token" = [])
    ),
)]
#[debug_handler]
#[instrument(level = "info", name = "create_dog", skip_all)]
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

#[utoipa::path(
    get,
    path = "/dogs",
    tag = "dogs",
    responses(
       (status = 200, description = "Hello World", body = DogListResponse),
    ),
)]
#[instrument(level = "info", name = "list_dogs", skip_all)]
pub async fn list(
    State(state): State<Arc<ApplicationState>>,
) -> Result<Json<DogListResponse>, AppError> {
    let dogs = Dog::find().all(state.db_conn.load().as_ref()).await?;
    let n = dogs.len();

    let response = DogListResponse {
        status: Status::Success,
        data: dogs,
    };

    tracing::info!("number of dogs: {}", n);

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/dogs/{dogId}",
    tag = "dogs",
    params(
        ("dogId" = i32, Path, description = "id of the dog"),
    ),
    responses(
        (status = 200, description = "Dog", body = DogGetResponse),
    ),
)]
#[instrument(level = "info", name = "get_dog", skip_all)]
pub async fn get(
    State(state): State<Arc<ApplicationState>>,
    Path(dog_id): Path<i32>,
) -> Result<Json<DogGetResponse>, AppError> {
    let dog = Dog::find_by_id(dog_id)
        .one(state.db_conn.load().as_ref())
        .await?;

    let response = DogGetResponse {
        status: Status::Success,
        data: dog,
    };

    Ok(Json(response))
}
