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