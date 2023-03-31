use std::{sync::Arc, error::Error, time::{SystemTime, UNIX_EPOCH}};

use jsonwebtoken::{encode, Header, Algorithm, EncodingKey, decode, Validation, DecodingKey};

use crate::storage::user_tokens::{UserTokenRepository, UserTokenDto};

use super::{AccessTokenModel, TokensState};

pub struct TokenService {
    state: Arc<TokensState>,
    user_token_repository: Arc<dyn UserTokenRepository + Sync + Send>,
}

impl TokenService {
    pub fn new(
        state: Arc<TokensState>,
        user_token_repository: Arc<dyn UserTokenRepository + Sync + Send>,
    ) -> Self {
        Self {
            state,
            user_token_repository,
        }
    }

    pub async fn create_refresh(&self, user_id: i64) -> Result<(String, i64), Box<dyn Error>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64;

        let refresh_token = UserTokenDto::new(user_id);

        let ttl = (self.state.refresh_lifetime / 1000) as i32;
        self.user_token_repository.create(&refresh_token, ttl).await?;

        let token = format!(
            "{}:{}", 
            base64_url::encode(&refresh_token.user_id.to_le_bytes()), 
            &refresh_token.id
        );

        let expires_at = now + self.state.refresh_lifetime;

        Ok((token, expires_at))
    }

    pub fn create_access(&self, user_id: i64) -> Result<(String, i64), Box<dyn Error>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64;

        let expires_at = now + self.state.access_lifetime;

        let claims = AccessTokenModel {
            sub: user_id,
            exp: expires_at,
            iat: now,
            nbf: now,
            iss: base64_url::encode(&self.state.instance_id.as_bytes()),
            aud: None,
        };
        
        let token = encode(
            &Header::new(Algorithm::ES384), 
            &claims, 
            &EncodingKey::from_ec_pem(self.state.jwt_private_key.as_bytes())?,
        )?;

        Ok((token, expires_at))
    }

    pub async fn find_refresh(&self, token: &str) -> Result<Option<i64>, Box<dyn Error>> {
        let parts: Vec<&str> = token.split(':').collect();
        if parts.len() != 2 {
            return Ok(None)
        }

        if let Ok(id_bytes) = base64_url::decode(parts[0]) {
            if id_bytes.len() != 8 {
                return Ok(None)
            }

            let id = i64::from_le_bytes(id_bytes[0..8].try_into().unwrap());
            let dto = UserTokenDto { 
                user_id: id, 
                id: String::from(parts[1]), 
            };

            if self.user_token_repository.exists(&dto).await? {
                return Ok(Some(id))
            }
        }

        Ok(None)
    }

    pub fn decode_access(&self, token: &str) -> Result<Option<AccessTokenModel>, Box<dyn Error>> {
        let result = decode::<AccessTokenModel>(
            token, 
            &DecodingKey::from_ec_pem(self.state.jwt_public_key.as_bytes())?,
            &Validation::new(Algorithm::ES384)
        );

        match result {
            Ok(model) => Ok(Some(model.claims)),
            Err(_) => Ok(None)
        }
    }
}