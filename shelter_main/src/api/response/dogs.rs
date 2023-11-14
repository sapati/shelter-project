use entity::dog::Model;
use serde::Serialize;

#[derive(Serialize)]
pub struct DogCreateResponse {
    pub status: String,
    pub data: Option<Model>,
}
