use super::handlers;
use crate::api::middleware::jwt::auth;
use crate::state::ApplicationState;
use axum::routing::{get, post};
use axum::{middleware, Router};
use std::sync::Arc;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::hello::hello,
        handlers::login::login,
        handlers::dogs::create,
        handlers::dogs::list,
        handlers::dogs::get,
    ),
    components(
        schemas(
            crate::api::request::login::LoginRequest,
            crate::api::response::login::LoginResponse,
            crate::api::response::error::ErrorResponse,
            crate::api::response::error::Status,
            crate::api::response::dogs::DogGetResponse,
            crate::api::response::dogs::DogListResponse,
            crate::api::response::dogs::DogCreateResponse,
            entity::dog::Model,
            entity::dog::DogCreateRequest,
        ),
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "hello", description = "Hello"),
        (name = "login", description = "Login"),
        (name = "dogs", description = "Dogs"),
    ),
    servers(
        (url = "/v1", description = "Local server"),
    ),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there are already components registered.
        components.add_security_scheme(
            "api_jwt_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route(
            "/hello",
            get(handlers::hello::hello).with_state(state.clone()),
        )
        .route(
            "/login",
            post(handlers::login::login).with_state(state.clone()),
        )
        .route(
            "/dogs",
            post(handlers::dogs::create)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
        .route("/dogs", get(handlers::dogs::list).with_state(state.clone()))
        .route("/dogs/:id", get(handlers::dogs::get).with_state(state))
}
