use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, message = "Email phải ít nhất 3 ký tự"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password phải ít nhất 6 ký tự"))]
    pub password: String,
}
