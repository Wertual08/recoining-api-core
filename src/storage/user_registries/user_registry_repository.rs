use std::{error::Error, fmt};

use tonic::async_trait;

use super::UserRegistryDto;

#[async_trait]
pub trait UserRegistryRepository: fmt::Debug {
    async fn list(&self, user_id: i64, last_updated_at: i64, limit: i32) -> Result<Vec<UserRegistryDto>, Box<dyn Error>>;
}