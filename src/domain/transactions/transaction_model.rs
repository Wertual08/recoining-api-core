use std::time::{UNIX_EPOCH, SystemTime};

use bigdecimal::{BigDecimal, num_bigint::Sign};
use sha2::{Sha256, Digest};

use crate::storage::transactions::TransactionDto;

use super::transaction_variant_model::TransactionVariantModel;

#[derive(Clone)]
pub struct TransactionModel {
    pub registry_id: i64,
    pub pack: i64,
    pub created_at: i64,
    pub source_user_id: i64,
    pub target_user_id: i64,
    pub sequence: i16,
    pub variant: TransactionVariantModel,
    pub amount: BigDecimal,
    pub currency: String,
    pub label: String,
    pub description: String,
    pub hash: Vec<u8>,
}

impl TransactionModel {
    pub fn basic(
        registry_id: i64, 
        source_user_id: i64,
        target_user_id: i64,
        amount: BigDecimal,
        currency: String,
        label: String,
        description: String,
        previous: &Option<Self>,
    ) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let (pack, sequence) = if let Some(previous) = previous {
            Self::next(previous.pack, previous.sequence)
        }
        else {
            (0, 0)
        };

        let mut result = Self {
            registry_id,
            pack,
            created_at,
            source_user_id,
            target_user_id,
            sequence,
            variant: TransactionVariantModel::Basic,
            amount,
            currency,
            label,
            description,
            hash: Vec::new(),
        };

        result.hash = if let Some(transaction) = previous {
            result.hash(&transaction.hash)
        }
        else {
            result.hash(&Vec::new())
        };

        result
    }

    pub fn next(pack: i64, sequence: i16) -> (i64, i16) {
        if sequence < i16::MAX {
            (pack, sequence + 1)
        }
        else {
            (pack + 1, 0)
        }
    }

    pub fn hash(&self, previous: &Vec<u8>) -> Vec<u8> {
        let (bigint, amaount_exponent) = self.amount.as_bigint_and_exponent();
        
        let (amount_sign, amount_bytes) = bigint.to_bytes_le();
        let curency_bytes = self.currency.as_bytes();
        let label_bytes = self.currency.as_bytes();
        let description_bytes = self.currency.as_bytes();
        
        let mut content = Vec::with_capacity(53 
            + amount_bytes.len() 
            + curency_bytes.len() 
            + label_bytes.len() 
            + description_bytes.len()
            + previous.len()
        );

        content.extend_from_slice(&self.registry_id.to_le_bytes());
        content.extend_from_slice(&self.pack.to_le_bytes());
        content.extend_from_slice(&self.created_at.to_le_bytes());
        content.extend_from_slice(&self.source_user_id.to_le_bytes());
        content.extend_from_slice(&self.target_user_id.to_le_bytes());
        content.extend_from_slice(&self.sequence.to_le_bytes());
        content.extend_from_slice(&i16::from(self.variant).to_le_bytes());
        content.push(match amount_sign {
            Sign::Minus => 1,
            Sign::NoSign => 2,
            Sign::Plus => 3,
        });
        content.extend_from_slice(&amount_bytes);
        content.extend_from_slice(&amaount_exponent.to_le_bytes());
        content.extend_from_slice(&curency_bytes);
        content.extend_from_slice(&label_bytes);
        content.extend_from_slice(&description_bytes);
        content.extend_from_slice(&previous);

        let mut hasher = Sha256::new();
        hasher.update(&content);
        hasher.finalize().to_vec()
    }
}

impl From<TransactionDto> for TransactionModel {
    fn from(dto: TransactionDto) -> Self {
        Self {
            registry_id: dto.registry_id,
            pack: dto.pack,
            created_at: dto.created_at,
            source_user_id: dto.source_user_id,
            target_user_id: dto.target_user_id,
            sequence: dto.sequence,
            variant: dto.variant.into(),
            amount: dto.amount,
            currency: dto.currency,
            label: dto.label,
            description: dto.description,
            hash: dto.hash,
        }
    }
}

impl From<TransactionModel> for TransactionDto {
    fn from(model: TransactionModel) -> Self {
        Self {
            registry_id: model.registry_id,
            pack: model.pack,
            created_at: model.created_at,
            source_user_id: model.source_user_id,
            target_user_id: model.target_user_id,
            sequence: model.sequence,
            variant: model.variant.into(),
            amount: model.amount,
            currency: model.currency,
            label: model.label,
            description: model.description,
            hash: model.hash,
        }
    }
}