use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokensConfig {
    pub jwt_private_key_path: String,
    pub jwt_public_key_path: String,
    pub refresh_lifetime: i64,
    pub access_lifetime: i64,
}