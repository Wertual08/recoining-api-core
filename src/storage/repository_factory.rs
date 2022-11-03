use std::{sync::Arc, error::Error};

use super::{phone_codes::{PhoneCodeRepository, ScyllaPhoneCodeRepository}, ScyllaContext, users::{UserRepository, ScyllaUserRepository}, user_tokens::{UserTokenRepository, ScyllaUserTokenRepository}};

#[derive(Debug)]
pub struct RepositoryFactory {
    phone_code_repository: Arc<dyn PhoneCodeRepository + Sync + Send>,
    user_repository: Arc<dyn UserRepository + Sync + Send>,
    user_token_repository: Arc<dyn UserTokenRepository + Sync + Send>,
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
}