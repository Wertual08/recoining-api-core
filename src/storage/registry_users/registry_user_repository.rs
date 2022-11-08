use std::{error::Error, fmt};

use tonic::async_trait;

use super::{RegistryUserDto, RegistryUserUpdateDto};

#[async_trait]
pub trait RegistryUserRepository: fmt::Debug {
    async fn create(&self, dtos: &[RegistryUserDto]) -> Result<bool, Box<dyn Error>>;
    async fn update(&self, dtos: &[RegistryUserUpdateDto]) -> Result<bool, Box<dyn Error>>;
    async fn list(&self, registry_id: i64, user_ids: &[i64]) -> Result<Vec<RegistryUserDto>, Box<dyn Error>>;
    async fn count(&self, registry_id: i64, user_ids: &[i64]) -> Result<i64, Box<dyn Error>>;
}