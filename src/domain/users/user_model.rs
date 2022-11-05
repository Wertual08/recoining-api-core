use std::collections::HashMap;

use bigdecimal::BigDecimal;

use crate::storage::users::UserDto;

pub struct UserModel {
    pub id: i64,
    pub phone: i64,
    pub email: String,
    pub login: String,
    pub avatar: String,
    pub balance: HashMap<String, BigDecimal>,
}

impl From<UserDto> for UserModel {
    fn from(user_dto: UserDto) -> Self {
        Self {
            id: user_dto.id,
            phone: user_dto.phone,
            email: user_dto.email,
            login: user_dto.login,
            avatar: user_dto.avatar,
            balance: user_dto.balance,
        }
    }
}