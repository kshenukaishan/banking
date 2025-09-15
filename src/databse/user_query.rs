use bcrypt::{hash, DEFAULT_COST};
use sqlx::MySqlPool;
use crate::models::sign_up_request::SignUpRequest;
use models::user_model::User;
use crate::models;

pub async fn has_email_already(pool: &MySqlPool, email: &str) -> bool {
    let row = sqlx::query!("SELECT EXISTS(SELECT 1 FROM users WHERE email = ?) AS exists_val", email)
    .fetch_one(pool)
    .await
    .unwrap();
    row.exists_val != 0
}

pub async fn create_user(pool: &MySqlPool, user: &SignUpRequest) -> bool {
    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();
    sqlx::query!("INSERT INTO users(`email`,`first_name`,`last_name`,`password`) VALUES (?, ?, ?, ?)",
        &user.email, &user.first_name, &user.last_name, &hashed_password)
    .execute(pool)
    .await.is_ok()
}

pub async fn check_user_exist(pool: &MySqlPool, email: &str) -> Option<User> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
        .fetch_optional(pool)
        .await
        .unwrap()
}