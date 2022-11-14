use std::collections::HashMap;

use bigdecimal::BigDecimal;

use crate::storage::registry_users::RegistryUserDto;

pub struct RegistryUserModel {
    pub registry_id: i64,
    pub user_id: i64,
    pub updated_at: i64,
    pub current_pack: i64,
    pub current_sequence: i16,
    pub balance: HashMap<String, BigDecimal>,
}

impl From<RegistryUserDto> for RegistryUserModel {
    fn from(dto: RegistryUserDto) -> Self {
        Self {
            registry_id: dto.registry_id,
            user_id: dto.user_id,
            updated_at: dto.updated_at,
            current_pack: dto.current_pack,
            current_sequence: dto.current_sequence,
            balance: dto.balance,
        }
    }
}