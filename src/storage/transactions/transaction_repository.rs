use std::{error::Error, fmt};

use tonic::async_trait;

use super::TransactionDto;

#[async_trait]
pub trait TransactionRepository: fmt::Debug {
    async fn create(&self, dto: &TransactionDto) -> Result<bool, Box<dyn Error>>;
    async fn find_last(&self, registry_id: i64, pack: i64) -> Result<Option<TransactionDto>, Box<dyn Error>>;
    async fn list(&self, registry_id: i64, pack: i64, last_sequence: i16, limit: i32) -> Result<Vec<TransactionDto>, Box<dyn Error>>;
}