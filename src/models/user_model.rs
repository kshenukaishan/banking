use serde::Serialize;
use sqlx::types::chrono;

#[derive(Serialize, Debug)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub balance: u64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}