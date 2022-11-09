use std::collections::HashMap;

use bigdecimal::BigDecimal;

pub struct RegistryUserDto {
    pub registry_id: i64,
    pub user_id: i64,
    pub updated_at: i64,
    pub current_pack: i64,
    pub current_sequence: i16,
    pub balance: HashMap<String, BigDecimal>,
}

impl RegistryUserDto {
    pub fn new(timestamp: i64, registry_id: i64, user_id: i64) -> Self {
        Self {
            registry_id,
            user_id,
            updated_at: timestamp,
            current_pack: 0,
            current_sequence: -1,
            balance: HashMap::new(),
        }
    }
}