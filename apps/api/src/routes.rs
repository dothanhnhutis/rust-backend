use axum::{routing::post, Router};
use sqlx::PgPool;

use crate::handlers; // Gọi thông qua module handlers đã khai báo
pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/login", post(handlers::auth_handler::login))
        .with_state(pool)
}
