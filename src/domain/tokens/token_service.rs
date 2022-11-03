use std::{sync::Arc, error::Error, time::{SystemTime, UNIX_EPOCH}};

use crate::storage::user_tokens::{UserTokenRepository, UserTokenDto};

use super::TokensPayloadModel;

pub struct TokenService {
    refresh_token_lifetime: i64,
    access_token_lifetime: i64,
    user_token_repository: Arc<dyn UserTokenRepository + Sync + Send>,
}

impl TokenService {
    pub fn new(user_token_repository: Arc<dyn UserTokenRepository + Sync + Send>) -> Self {
        Self {
            refresh_token_lifetime: 31556926000,
            access_token_lifetime: 900000,
            user_token_repository,
        }
    }

    pub async fn create_refresh(&self, user_id: i64) -> Result<TokensPayloadModel, Box<dyn Error>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64;

        let refresh_token = UserTokenDto::new(user_id);

        let ttl = (self.refresh_token_lifetime / 1000) as i32;
        self.user_token_repository.create(&refresh_token, ttl).await?;

        let refresh_expires_at = now + self.refresh_token_lifetime;
        let access_expires_at = now + self.access_token_lifetime;

        let model = TokensPayloadModel { 
            refresh_token: format!("{}:{}", refresh_token.user_id, &refresh_token.id), 
            refresh_expires_at, 
            access_token: String::from("gayshit"), 
            access_expires_at,
        };

        Ok(model)
    }
}