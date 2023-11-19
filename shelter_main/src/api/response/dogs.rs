use crate::api::response::error::Status;
use entity::dog::Model;
use serde::Serialize;

#[derive(Serialize)]
pub struct DogCreateResponse {
    pub status: Status,
    pub data: Option<Model>,
}

#[derive(Serialize)]
pub struct DogListResponse {
    pub status: Status,
    pub data: Vec<Model>,
}

#[derive(Serialize)]
pub struct DogGetResponse {
    pub status: Status,
    pub data: Option<Model>,
}
