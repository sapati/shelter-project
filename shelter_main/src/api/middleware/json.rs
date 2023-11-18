use crate::api::response::error::Status;
use axum::http::Request;
use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest},
    http::StatusCode,
};
use serde_json::{json, Value};

pub struct CustomJson<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for CustomJson<T>
where
    axum::Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err((
                rejection.status(),
                axum::Json(json!({
                    "status": Status::Error,
                    "message": rejection.body_text(),
                })),
            )),
        }
    }
}
