use application::dto::auth::login_request::LoginRequest;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use sqlx::PgPool;

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // Logic xử lý ở đây, gọi sang infrastructure::repositories::user_repository

    // 1. Validate dữ liệu đầu vào
    if let Err(e) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        );
    }

    // 2. Tìm user trong DB
    let user_result = find_by_email(&pool, &payload.email).await;

    match user_result {
        Ok(Some(user)) => {
            // 3. Kiểm tra mật khẩu (Ở đây tôi ví dụ so sánh thẳng,
            // thực tế bạn phải dùng thư viện argon2 để verify password_hash)
            if user.password_hash == payload.password {
                (
                    StatusCode::OK,
                    Json(json!({
                        "message": "Đăng nhập thành công",
                        "user_id": user.id,
                        "role": user.role
                    })),
                )
            } else {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "Sai mật khẩu" })),
                )
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User không tồn tại" })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Lỗi hệ thống" })),
        ),
    }
}
