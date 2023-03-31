use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CodesConfig {
    pub attemtps_phone: i16,
    pub max_phone: i64,
    pub timeout_phone: i64,
    pub expiration_phone: i64,
}