use std::sync::Arc;

use axum::Router;

use crate::AppState;

mod auth_route;

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/auth", auth_route::create_auth_route())
}
