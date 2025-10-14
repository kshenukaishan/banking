use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UpdateAccountRequest {
    pub name: String,
    pub description: String,
    pub balance: u64,
}