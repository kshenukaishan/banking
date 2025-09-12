use sqlx::MySqlPool;
use crate::models::sign_up_request::SignUpRequest;

pub async fn has_email_already(pool: &MySqlPool, email: &str) -> bool {
    let row = sqlx::query!("SELECT EXISTS(SELECT 1 FROM users WHERE email = ?) AS exists_val", email)
    .fetch_one(pool)
    .await
    .unwrap();
    row.exists_val != 0
}

pub async fn create_user(pool: &MySqlPool, user: &SignUpRequest) -> bool {
    sqlx::query!("INSERT INTO users(`email`,`first_name`,`last_name`,`password`) VALUES (?, ?, ?, ?)",
        &user.email, &user.password, &user.first_name, &user.last_name)
    .execute(pool)
    .await.is_ok()
}