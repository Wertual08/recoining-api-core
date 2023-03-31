use std::{error::Error, fs::File, io::Read};

use uuid::Uuid;

use super::TokensConfig;

#[derive(Debug)]
pub struct TokensState {
    pub instance_id: Uuid,
    pub jwt_private_key: String,
    pub jwt_public_key: String,
    pub refresh_lifetime: i64,
    pub access_lifetime: i64,
}

impl TokensState {
    pub fn new(
        instance_id: Uuid, 
        config: &TokensConfig,
    ) -> Result<Self, Box<dyn Error>> {
        let mut jwt_private_key = String::new();
        let mut jwt_public_key = String::new();

        File::open(&config.jwt_private_key_path)?
            .read_to_string(&mut jwt_private_key)?;
        File::open(&config.jwt_public_key_path)?
            .read_to_string(&mut jwt_public_key)?;

        let result = Self {
            instance_id,
            jwt_private_key,
            jwt_public_key,
            refresh_lifetime: config.refresh_lifetime,
            access_lifetime: config.access_lifetime,
        };

        Ok(result)
    }
}