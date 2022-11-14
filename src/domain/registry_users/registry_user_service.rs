use std::{sync::Arc, error::Error};

use crate::storage::registry_users::RegistryUserRepository;

use super::RegistryUserModel;

pub struct RegistryUserService {
    registry_user_repository: Arc<dyn RegistryUserRepository + Sync + Send>,
}

impl RegistryUserService {
    pub fn new(
        registry_user_repository: Arc<dyn RegistryUserRepository + Sync + Send>,
    ) -> Self {
        Self {
            registry_user_repository,
        }
    }

    pub async fn list(&self, registry_id: i64, user_ids: &[i64]) -> Result<Vec<RegistryUserModel>, Box<dyn Error>> {
        let dtos = self.registry_user_repository.list(registry_id, user_ids).await?;
        Ok(dtos.into_iter().map(|dto| dto.into()).collect())
    }

    pub async fn count(&self, registry_id: i64, user_ids: &[i64]) -> Result<i64, Box<dyn Error>> {
        Ok(self.registry_user_repository.count(registry_id, user_ids).await?)
    }
}