// use domain::User;
// use sqlx::PgPool;

// pub async fn find_by_email(pool: &PgPool, email: &str) -> anyhow::Result<Option<User>> {
//     // Chúng tự map tay hoặc dùng macro query_as mà không cần FromRow trong Domain
//     let user = sqlx::query_as!(
//         User,
//         "SELECT id, email, password_hash FROM users WHERE email = $1",
//         email
//     )
//     .fetch_optional(pool)
//     .await?;
//     Ok(user)
// }

use anyhow::Result;
use domain::User;
use sqlx::PgPool;

// Struct dùng riêng cho DB (infrastructure)
#[derive(Debug, sqlx::FromRow)]
struct UserRow {
    pub id: String,
    pub email: String,
    pub password_hash: String,
}

// Mapping từ DB → Domain
// impl From<UserRow> for User {
//     fn from(row: UserRow) -> Self {
//         Self {
//             id: row.id,
//             email: row.email,
//             password_hash: row.password_hash,
//         }
//     }
// }

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<UserRow>> {
    let row = sqlx::query_as!(
        UserRow,
        r#"
        SELECT id, email, password_hash
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(row)
}
