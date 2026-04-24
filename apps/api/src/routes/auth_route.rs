use crate::{handlers::auth_handler::login, AppState};
use axum::{routing::post, Router};

pub fn create_auth_route() -> Router<AppState> {
    Router::new().route("/login", post(login))
}
