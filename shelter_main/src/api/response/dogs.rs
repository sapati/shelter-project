use crate::api::response::error::Status;
use entity::dog::Model;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct DogCreateResponse {
    pub status: Status,
    pub data: Option<Model>,
}

#[derive(Serialize, ToSchema)]
pub struct DogListResponse {
    pub status: Status,
    pub data: Vec<Model>,
}

#[derive(Serialize, ToSchema)]
pub struct DogGetResponse {
    pub status: Status,
    pub data: Option<Model>,
}
