use axum::{extract::FromRef, Router};
use dotenvy::dotenv;
use infrastructure::init_db_pool;
use sqlx::PgPool;
use std::env;

mod handlers;
mod routes;

#[derive(Clone, FromRef)]
struct AppState {
    db: PgPool,
}

// impl FromRef<AppState> for Pool<Postgres> {
//     fn from_ref(state: &AppState) -> Self {
//         state.db.clone()
//     }
// }

#[tokio::main]
async fn main() {
    dotenv().ok();

    // 1. Khởi tạo Log
    tracing_subscriber::fmt::init();

    // 2. Kết nối Database từ Lib Infrastructure
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = init_db_pool(&db_url)
        .await
        .expect("Failed to connect to DB");

    let shared_state = AppState { db: pool };

    let app = Router::new()
        .nest("/api/v1", routes::create_router())
        .with_state(shared_state);

    // 4. Chạy server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Server đang chạy tại: http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}
