use std::{sync::Arc, error::Error};

use super::{
    phone_codes::{PhoneCodeRepository, ScyllaPhoneCodeRepository}, 
    ScyllaContext, 
    users::{UserRepository, ScyllaUserRepository}, 
    user_tokens::{UserTokenRepository, ScyllaUserTokenRepository}, 
    registries::{RegistryRepository, ScyllaRegistryRepository}, 
    registry_users::{RegistryUserRepository, ScyllaRegistryUserRepository}, 
    user_registries::{UserRegistryRepository, ScyllaUserRegistryRepository}, 
    transactions::{TransactionRepository, ScyllaTransactionRepository}
};

#[derive(Debug)]
pub struct RepositoryFactory {
    phone_code_repository: Arc<dyn PhoneCodeRepository + Sync + Send>,
    user_repository: Arc<dyn UserRepository + Sync + Send>,
    user_token_repository: Arc<dyn UserTokenRepository + Sync + Send>,
    registry_repository: Arc<dyn RegistryRepository + Sync + Send>,
    registry_user_repository: Arc<dyn RegistryUserRepository + Sync + Send>,
    user_registry_repository: Arc<dyn UserRegistryRepository + Sync + Send>,
    transaction_repository: Arc<dyn TransactionRepository + Sync + Send>,
}

impl RepositoryFactory {
    pub async fn new(scylla_context: &Arc<ScyllaContext>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            phone_code_repository: Arc::new(
                ScyllaPhoneCodeRepository::new(Arc::clone(scylla_context)).await?
            ),
            user_repository: Arc::new(
                ScyllaUserRepository::new(Arc::clone(scylla_context)).await?
            ),
            user_token_repository: Arc::new(
                ScyllaUserTokenRepository::new(Arc::clone(scylla_context)).await?
            ),
            registry_repository: Arc::new(
                ScyllaRegistryRepository::new(Arc::clone(scylla_context)).await?
            ),
            registry_user_repository: Arc::new(
                ScyllaRegistryUserRepository::new(Arc::clone(scylla_context)).await?
            ),
            user_registry_repository: Arc::new(
                ScyllaUserRegistryRepository::new(Arc::clone(scylla_context)).await?
            ),
            transaction_repository: Arc::new(
                ScyllaTransactionRepository::new(Arc::clone(scylla_context)).await?
            ),
        })
    }

    pub fn phone_code(&self) -> Arc<dyn PhoneCodeRepository + Sync + Send> {
        Arc::clone(&self.phone_code_repository)
    }

    pub fn user(&self) -> Arc<dyn UserRepository + Sync + Send> {
        Arc::clone(&self.user_repository)
    }

    pub fn user_token(&self) -> Arc<dyn UserTokenRepository + Sync + Send> {
        Arc::clone(&self.user_token_repository)
    }

    pub fn registry(&self) -> Arc<dyn RegistryRepository + Sync + Send> {
        Arc::clone(&self.registry_repository)
    }

    pub fn registry_user(&self) -> Arc<dyn RegistryUserRepository + Sync + Send> {
        Arc::clone(&self.registry_user_repository)
    }

    pub fn user_registry(&self) -> Arc<dyn UserRegistryRepository + Sync + Send> {
        Arc::clone(&self.user_registry_repository)
    }

    pub fn transaction(&self) -> Arc<dyn TransactionRepository + Sync + Send> {
        Arc::clone(&self.transaction_repository)
    }
}