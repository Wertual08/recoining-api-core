use std::{fmt, error::Error};

use tonic::async_trait;

use super::UserDto;


#[async_trait]
pub trait UserRepository: fmt::Debug {
    async fn create(&self, dto: &UserDto) -> Result<bool, Box<dyn Error>>;

    async fn find_phone(&self, phone: i64) -> Result<Option<UserDto>, Box<dyn Error>>;
}