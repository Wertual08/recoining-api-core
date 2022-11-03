use std::sync::{Arc, Mutex};

use crate::storage::{RepositoryFactory, id_generator::IdGenerator};

use super::{codes::CodeService, users::UserService, tokens::TokenService};

#[derive(Debug)]
pub struct ServiceFactory {
    id_generator: Arc<Mutex<IdGenerator>>,
    repository_factory: RepositoryFactory,
}

impl ServiceFactory {
    pub fn new(repository_factory: RepositoryFactory) -> Self {
        Self {
            id_generator: Arc::new(Mutex::new(IdGenerator::new(0))),
            repository_factory: repository_factory,
        }
    }

    pub fn code(&self) -> CodeService {
        CodeService::new(self.repository_factory.phone_code())
    }

    pub fn user(&self) -> UserService {
        UserService::new(
            Arc::clone(&self.id_generator),
            self.repository_factory.user(),
        )  
    }

    pub fn token(&self) -> TokenService {
        TokenService::new(
            self.repository_factory.user_token(),
        )  
    }
}