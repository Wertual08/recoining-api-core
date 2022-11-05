use std::collections::HashMap;

use bigdecimal::BigDecimal;

pub struct UserDto {
    pub id: i64,
    pub phone: i64,
    pub email: String,
    pub login: String,
    pub avatar: String,
    pub balance: HashMap<String, BigDecimal>,
}

impl UserDto {
    pub fn from_phone(id: i64, phone: i64) -> Self {
        Self {
            id,
            phone,
            email: String::new(),
            login: String::new(),
            avatar: String::new(),
            balance: HashMap::new(),
        }
    }
}