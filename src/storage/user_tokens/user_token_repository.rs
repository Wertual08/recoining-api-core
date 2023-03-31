use std::{error::Error, fmt};

use tonic::async_trait;

use super::UserTokenDto;

#[async_trait]
pub trait UserTokenRepository: fmt::Debug {
    async fn create(&self, dto: &UserTokenDto, ttl: i32) -> Result<(), Box<dyn Error>>;
    async fn exists(&self, dto: &UserTokenDto) -> Result<bool, Box<dyn Error>>;
}