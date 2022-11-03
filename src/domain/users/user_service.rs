use std::{sync::{Arc, Mutex}, error::Error};

use crate::storage::{id_generator::{IdGenerator}, users::{UserRepository, UserDto}};

pub struct UserService {
    id_generator: Arc<Mutex<IdGenerator>>,
    user_repository: Arc<dyn UserRepository + Sync + Send>,
}

impl UserService {
    pub fn new(
        id_generator: Arc<Mutex<IdGenerator>>,
        user_repository: Arc<dyn UserRepository + Sync + Send>,
    ) -> Self {
        Self {
            id_generator,
            user_repository,
        }
    }

    pub async fn get_id_phone(&self, phone: i64) -> Result<Option<i64>, Box<dyn Error>> {
        let dto_option = self.user_repository.find_phone(phone).await?;

        match dto_option {
            Some(dto) => Ok(Some(dto.id)),
            None => {
                let dto = UserDto::from_phone(
                    self.id_generator.lock().unwrap().create(), 
                    phone,
                );

                if self.user_repository.create(&dto).await? {
                    Ok(Some(dto.id))
                }
                else {
                    Ok(None)
                }
            }
        }
    }  
}