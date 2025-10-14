use actix_web::web;
use sqlx::MySqlPool;
use crate::AppState;
use crate::models::account::Account;
use crate::models::update_account_request::UpdateAccountRequest;

pub async fn create_account(pool: &MySqlPool, data: web::Json<Account>) -> bool {
    sqlx::query!("INSERT INTO `accounts` (`user_id`, `name`, `description`, `balance`) VALUES (?,?,?,?)",
    data.user_id, data.name, data.description, data.balance)
        .execute(pool).await.is_ok()
}

pub async fn get_account(pool: &MySqlPool, id: u64) -> Account {
    sqlx::query_as!(Account, "SELECT * FROM accounts WHERE id = ?", id)
    .fetch_one(pool)
    .await
    .unwrap()
}

pub async fn update(pool: &MySqlPool, data: &UpdateAccountRequest, id: u64) {
    sqlx::query!("UPDATE accounts SET name =  ?, description = ?, balance = ? WHERE id = ?",
        &data.name,
        &data.description,
        &data.balance,
        id)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn delete(pool: &MySqlPool, id: u64) {
    sqlx::query!("DELETE FROM accounts WHERE id = ?", id)
    .execute(pool)
    .await
    .unwrap();
}