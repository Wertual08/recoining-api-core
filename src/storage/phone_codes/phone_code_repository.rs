use std::{fmt, error::Error};

use tonic::async_trait;

use super::PhoneCodeDto;

#[async_trait]
pub trait PhoneCodeRepository: fmt::Debug {
    async fn create(&self, dto: &PhoneCodeDto) -> Result<bool, Box<dyn Error>>;
    async fn delete(&self, dto: &PhoneCodeDto) -> Result<bool, Box<dyn Error>>;
    async fn find(&self, phone: i64) -> Result<Option<PhoneCodeDto>, Box<dyn Error>>;
}