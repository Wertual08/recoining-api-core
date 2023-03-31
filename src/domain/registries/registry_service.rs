use std::{sync::{Arc, Mutex}, error::Error, time::{SystemTime, UNIX_EPOCH}};

use crate::storage::{
    registries::RegistryRepository, 
    id_generator::IdGenerator, 
    registry_users::{RegistryUserDto, RegistryUserRepository}, 
    user_registries::UserRegistryRepository,
};

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
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64;

        let registry = RegistryModel::direct(
            timestamp,
            self.id_generator.lock().unwrap().create(), 
            name, 
            image,
        );

        let registry_users = [
            RegistryUserDto::new(timestamp, registry.id, former_user_id),
            RegistryUserDto::new(timestamp, registry.id, second_user_id),
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
        Ok(registry.map(|dto| dto.into()))
    }

    pub async fn list_user_registries(
        &self,
        user_id: i64, 
        last_updated_at: i64, 
        limit: i32,
    ) -> Result<Vec<RegistryModel>, Box<dyn Error>> {
        let user_registries = self.user_registry_repository.list(
            user_id, 
            last_updated_at, 
            limit,
        ).await?;

        let registries = self.registry_repository.list(
            &user_registries.iter().map(|dto| dto.registry_id).collect::<Vec<i64>>()
        ).await?;

        Ok(registries.into_iter().map(|dto| dto.into()).collect())
    }
}