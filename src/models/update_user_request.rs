use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateUserRequest {
    pub first_name: String,
    pub last_name: String,
}

