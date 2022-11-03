use std::{sync::Arc, error::Error, time::{SystemTime, UNIX_EPOCH}};

use crate::storage::phone_codes::{PhoneCodeRepository, PhoneCodeDto};

use super::{CodeSendModel, CodeAttemptModel};

pub struct CodeService {
    code_attemtps_phone: i16,
    code_max_phone: i64,
    code_timeout_phone: i64,
    code_expiration_phone: i64,
    phone_code_repository: Arc<dyn PhoneCodeRepository + Sync + Send>,
}

impl CodeService {
    pub fn new(
        phone_code_repository: Arc<dyn PhoneCodeRepository + Sync + Send>,
    ) -> Self {
        Self {
            code_attemtps_phone: 3,
            code_max_phone: 1,
            code_timeout_phone: 60000,
            code_expiration_phone: 300000,
            phone_code_repository,
        }
    }

    pub async fn send_phone(&self, phone: i64) -> Result<CodeSendModel, Box<dyn Error>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let dto_option = self.phone_code_repository.find(phone).await?;

        let mut dto = if let Some(dto) = dto_option {
            let until = dto.created_at + self.code_timeout_phone;
            if until > now {
                return Ok(CodeSendModel::Timeout(until));
            }

            if !self.phone_code_repository.delete(&dto).await? {
                return Ok(CodeSendModel::Retry);
            }

            dto
        }
        else {
            PhoneCodeDto::new(
                phone, 
                self.code_max_phone, 
                (self.code_expiration_phone / 1000) as i32,
            )
        };

        dto.created_at = now;
        
        if !self.phone_code_repository.create(&dto).await? {
            return Ok(CodeSendModel::Retry);
        }

        // TODO: Send the code

        let result = CodeSendModel::Success(
            now + self.code_timeout_phone, 
            now + self.code_expiration_phone,
        );

        Ok(result)
    }

    pub async fn attempt_phone(&self, phone: i64, code: i64) -> Result<CodeAttemptModel, Box<dyn Error>> {
        let dto_option = self.phone_code_repository.find(phone).await?;

        if dto_option.is_none() {
            return Ok(CodeAttemptModel::Absent);
        }

        let mut dto = dto_option.unwrap();

        if !self.phone_code_repository.delete(&dto).await? {
            return Ok(CodeAttemptModel::Retry)
        }

        dto.attempts += 1;

        if dto.attempts > self.code_attemtps_phone {
            if !self.phone_code_repository.create(&dto).await? {
                return Ok(CodeAttemptModel::Retry)
            }
            
            return Ok(CodeAttemptModel::Fail(-1))
        }
        
        if dto.code == code {
            return Ok(CodeAttemptModel::Success);
        }

        if !self.phone_code_repository.create(&dto).await? {
            return Ok(CodeAttemptModel::Retry)
        }

        Ok(CodeAttemptModel::Fail(self.code_attemtps_phone - dto.attempts))
    }
}