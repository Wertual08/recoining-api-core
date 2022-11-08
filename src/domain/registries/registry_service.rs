use std::{sync::{Arc, Mutex}, error::Error};

use crate::storage::{registries::RegistryRepository, id_generator::IdGenerator, registry_users::{RegistryUserDto, RegistryUserRepository}, user_registries::UserRegistryRepository};

use super::RegistryModel;

pub struct RegistryService {
    id_generator: Arc<Mutex<IdGenerator>>,
    registry_repository: Arc<dyn RegistryRepository + Sync + Send>,
    registry_user_repository: Arc<dyn RegistryUserRepository + Sync + Send>,
    user_registry_repository: Arc<dyn UserRegistryRepository + Sync + Send>,
}

impl RegistryService {
    pub fn new(
        id_generator: Arc<Mutex<IdGenerator>>,
        registry_repository: Arc<dyn RegistryRepository + Sync + Send>,
        registry_user_repository: Arc<dyn RegistryUserRepository + Sync + Send>,
        user_registry_repository: Arc<dyn UserRegistryRepository + Sync + Send>,
    ) -> Self {
        Self {
            id_generator,
            registry_repository,
            user_registry_repository,
            registry_user_repository,
        }
    }

    pub async fn create_direct(
        &self, 
        former_user_id: i64, 
        second_user_id: i64,
        name: String,
        image: String,
    ) -> Result<Option<RegistryModel>, Box<dyn Error>> {
        let registry = RegistryModel::direct(
            self.id_generator.lock().unwrap().create(), 
            name, 
            image,
        );

        let registry_users = [
            RegistryUserDto::new(registry.id, former_user_id),
            RegistryUserDto::new(registry.id, second_user_id),
        ];

        if !self.registry_repository.create(&registry.clone().into()).await? {
            return Ok(None)
        }

        if !self.registry_user_repository.create(&registry_users).await? {
            return Ok(None)
        }

        Ok(Some(registry))
    }

    pub async fn access(&self, registry_id: i64, user_ids: &[i64]) -> Result<bool, Box<dyn Error>> {
        let count = self.registry_user_repository.count(registry_id, user_ids).await?;
        Ok(count == user_ids.len() as i64)
    }

    pub async fn find(&self, id: i64) -> Result<Option<RegistryModel>, Box<dyn Error>> {
        let registry = self.registry_repository.find(id).await?;
        Ok(registry.map(|model| model.into()))
    }
}