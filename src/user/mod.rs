use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    reputation: u32,
    account_id: u32,
    user_id: u32,
    user_type: String,
    display_name: String,
}
