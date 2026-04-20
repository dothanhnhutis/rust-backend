use dotenvy::dotenv;
use infrastructure::init_db_pool;
use std::env;

mod handlers;
mod routes;

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

    // 3. Build route
    // let app = Router::new()
    //     .route("/", get(|| async { "Hệ thống quản lý kho hóa chất sẵn sàng!" }))
    //     .with_state(pool); // Chia sẻ pool cho các handler

    let app = routes::create_router(pool);

    // 4. Chạy server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Server đang chạy tại: http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}
