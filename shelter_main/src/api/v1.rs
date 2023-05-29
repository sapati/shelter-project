use super::handlers;
use crate::state::ApplicationState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new().route("/hello", get(handlers::hello::hello).with_state(state))
}
