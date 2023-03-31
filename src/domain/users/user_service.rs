use std::{sync::{Arc, Mutex}, error::Error};

use crate::storage::{id_generator::{IdGenerator}, users::{UserRepository, UserDto}};

use super::UserModel;

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

    pub async fn find_id(&self, id: i64) -> Result<Option<UserModel>, Box<dyn Error>> {
        let dto_option = self.user_repository.find_id(id).await?;

        Ok(dto_option.map(|dto| UserModel::from(dto)))
    }

    pub async fn find_phone(&self, phone: i64) -> Result<Option<UserModel>, Box<dyn Error>> {
        let dto_option = self.user_repository.find_phone(phone).await?;

        Ok(dto_option.map(|dto| UserModel::from(dto)))
    }

    pub async fn find_email(&self, email: &String) -> Result<Option<UserModel>, Box<dyn Error>> {
        let dto_option = self.user_repository.find_email(email).await?;

        Ok(dto_option.map(|dto| UserModel::from(dto)))
    }
}