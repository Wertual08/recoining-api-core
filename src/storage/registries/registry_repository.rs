use std::{error::Error, fmt};

use tonic::async_trait;

use super::{RegistryDto, RegistryTransactionUpdateDto};

#[async_trait]
pub trait RegistryRepository: fmt::Debug {
    async fn create(&self, dto: &RegistryDto) -> Result<bool, Box<dyn Error>>;
    async fn update_transaction(&self, update_dto: &RegistryTransactionUpdateDto) -> Result<bool, Box<dyn Error>>;
    async fn find(&self, id: i64) -> Result<Option<RegistryDto>, Box<dyn Error>>;
    async fn list(&self, ids: &[i64]) -> Result<Vec<RegistryDto>, Box<dyn Error>>;
}