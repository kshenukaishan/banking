use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: u64,
    pub user_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub balance: u64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}