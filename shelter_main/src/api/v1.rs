use super::handlers;
use axum::routing::get;
use axum::Router;

pub fn configure() -> Router {
    Router::new().route("/hello", get(handlers::hello::hello))
}
