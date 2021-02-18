use crate::response_error::GenericResult;
use sqlx::MySqlPool;

pub struct User {
    pub email: String,
    pub username: String,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
}

pub async fn insert_user_unvalidated(pool: &MySqlPool, user: &User) -> GenericResult<()> {
    sqlx::query!(
        r#"
            INSERT INTO users(email, username, password_hash, password_salt)
            VALUES(?, ?, ?, ?)
        "#,
        user.email.to_ascii_lowercase(),
        user.username,
        &user.password_hash[0..],
        &user.password_salt[0..]
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn fetch_user(pool: &MySqlPool, email: &str) -> GenericResult<Option<User>> {
    Ok(sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email=?",
        email.to_ascii_lowercase()
    )
    .fetch_optional(pool)
    .await?)
}
