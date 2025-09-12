use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}