use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;
use sqlx::{prelude::FromRow, PgPool};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, message = "Password phải ít nhất 8 ký tự"))]
    pub password: String,
}

#[derive(FromRow)]
struct UserRow {
    id: String,
    email: String,
    username: String,
    password_hash: String,
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // validate
    if let Err(e) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        );
    }

    // query user
    let user_result = sqlx::query_as!(
        UserRow,
        r#"
        SELECT id, email, username, password_hash
        FROM users
        WHERE email = $1
        "#,
        payload.email
    )
    .fetch_optional(&pool)
    .await;

    // handle result
    let user = match user_result {
        Ok(user) => user,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            );
        }
    };

    // check user tồn tại
    let user = match user {
        Some(u) => u,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Email không tồn tại" })),
            );
        }
    };

    // TODO: verify password ở đây

    (
        StatusCode::OK,
        Json(json!({
            "message": "Login thành công",
            "user_id": user.id
        })),
    )
    // 1. Validate dữ liệu đầu vào
    // if let Err(e) = payload.validate() {
    //     return (
    //         StatusCode::BAD_REQUEST,
    //         Json(json!({ "error": e.to_string() })),
    //     );
    // }

    // 2. Tìm user trong DB
    // let user_result = find_by_email(&pool, &payload.email).await;

    // match user_result {
    //     Ok(Some(user)) => {
    //         // 3. Kiểm tra mật khẩu (Ở đây tôi ví dụ so sánh thẳng,
    //         // thực tế bạn phải dùng thư viện argon2 để verify password_hash)
    //         if user.password_hash == payload.password {
    //             (
    //                 StatusCode::OK,
    //                 Json(json!({
    //                     "message": "Đăng nhập thành công",
    //                     "user_id": user.id,
    //                     "role": user.role
    //                 })),
    //             )
    //         } else {
    //             (
    //                 StatusCode::UNAUTHORIZED,
    //                 Json(json!({ "error": "Sai mật khẩu" })),
    //             )
    //         }
    //     }
    //     Ok(None) => (
    //         StatusCode::NOT_FOUND,
    //         Json(json!({ "error": "User không tồn tại" })),
    //     ),
    //     Err(_) => (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         Json(json!({ "error": "Lỗi hệ thống" })),
    //     ),
    // }
}
