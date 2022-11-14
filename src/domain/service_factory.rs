use std::{sync::{Arc, Mutex}, error::Error};

use uuid::Uuid;

use crate::storage::{RepositoryFactory, id_generator::IdGenerator};

use super::{codes::CodeService, users::UserService, tokens::{TokenService, TokensState}, ServicesConfig, registries::RegistryService, registry_users::RegistryUserService, transactions::TransactionService};

#[derive(Debug)]
pub struct ServiceFactory {
    config: ServicesConfig,
    id_generator: Arc<Mutex<IdGenerator>>,
    tokens_state: Arc<TokensState>,
    repository_factory: RepositoryFactory,
}

impl ServiceFactory {
    pub fn new(
        config: ServicesConfig,
        repository_factory: RepositoryFactory
    ) -> Result<Self, Box<dyn Error>> {
        let instance_id = Uuid::new_v4();

        let result = Self {
            id_generator: Arc::new(
                Mutex::new(
                    IdGenerator::new(
                        (instance_id.as_u128() % 0x3ff) as i64
                    )
                )
            ),
            tokens_state: Arc::new(TokensState::new(instance_id, &config.tokens)?),
            repository_factory: repository_factory,
            config,
        };

        Ok(result)
    }

    pub fn code(&self) -> CodeService {
        CodeService::new(
            &self.config.codes,
            self.repository_factory.phone_code(),
        )
    }

    pub fn user(&self) -> UserService {
        UserService::new(
            Arc::clone(&self.id_generator),
            self.repository_factory.user(),
        )  
    }

    pub fn token(&self) -> TokenService {
        TokenService::new(
            Arc::clone(&self.tokens_state),
            self.repository_factory.user_token(),    
        )  
    }

    pub fn registry(&self) -> RegistryService {
        RegistryService::new(
            Arc::clone(&self.id_generator),
            self.repository_factory.registry(),    
            self.repository_factory.registry_user(),    
            self.repository_factory.user_registry(),    
        )  
    }

    pub fn registry_user(&self) -> RegistryUserService {
        RegistryUserService::new(
            self.repository_factory.registry_user(), 
        )  
    }

    pub fn transaction(&self) -> TransactionService {
        TransactionService::new(
            self.repository_factory.transaction(),
            self.repository_factory.registry(),
            self.repository_factory.registry_user(),
        )
    }
}