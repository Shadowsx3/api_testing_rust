use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthErrorResponse {
    pub reason: String,
}
