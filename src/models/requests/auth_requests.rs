use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}