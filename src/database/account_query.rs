use actix_web::web;
use sqlx::MySqlPool;
use crate::AppState;
use crate::models::account::Account;

pub async fn create_account(pool: &MySqlPool, data: web::Json<Account>) -> bool {
    sqlx::query!("INSERT INTO `accounts` (`user_id`, `name`, `description`, `balance`) VALUES (?,?,?,?)",
    data.user_id, data.name, data.description, data.balance)
        .execute(pool).await.is_ok()
}

pub async fn get_account(pool: &MySqlPool, data: web::Data<AppState>, id: u64) -> Option<Account> {
    sqlx::query_as!(Account, "SELECT * FROM accounts WHERE id = ?", id)
    .fetch_optional(pool)
    .await
    .unwrap()
}