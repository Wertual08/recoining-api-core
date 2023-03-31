use std::{sync::Arc, error::Error};

use bigdecimal::{BigDecimal, Zero};

use crate::{
    storage::{
        transactions::TransactionRepository, 
        registry_users::{RegistryUserRepository, RegistryUserUpdateDto}, 
        registries::{RegistryTransactionUpdateDto, RegistryRepository}
    }, 
    domain::{registries::RegistryModel, registry_users::RegistryUserModel},
};

use super::{TransactionModel, TransactionStateModel};

pub struct TransactionService {
    transaction_repository: Arc<dyn TransactionRepository + Sync + Send>,
    registry_repository: Arc<dyn RegistryRepository + Sync + Send>,
    registry_user_repository: Arc<dyn RegistryUserRepository + Sync + Send>,
}

impl TransactionService {
    pub fn new(
        transaction_repository: Arc<dyn TransactionRepository + Send + Sync>,
        registry_repository: Arc<dyn RegistryRepository + Sync + Send>,
        registry_user_repository: Arc<dyn RegistryUserRepository + Sync + Send>,
    ) -> Self {
        Self {
            transaction_repository,
            registry_repository,
            registry_user_repository,
        }
    }

    pub async fn send_basic(
        &self,
        registry: &RegistryModel,
        source_user_id: i64,
        target_user_id: i64,
        amount: BigDecimal,
        currency: String,
        label: String,
        description: String,
    ) -> Result<TransactionStateModel, Box<dyn Error>> {
        let last_transaction_option = self.find_last(registry).await?;
    
        if let Some(last_transaction) = last_transaction_option.as_ref() {
            if !self.ensure_complete(registry, last_transaction).await? {
                return Ok(TransactionStateModel::Fail);
            }
        }

        let transaction = TransactionModel::basic(
            registry.id, 
            source_user_id, 
            target_user_id, 
            amount, 
            currency, 
            label,
            description, 
            &last_transaction_option,
        );

        if !self.transaction_repository.create(&transaction.clone().into()).await? {
            return Ok(TransactionStateModel::Fail);
        }

        if !self.update_registry(registry, &transaction).await? {
            return Ok(TransactionStateModel::Pending(transaction));
        }

        if !self.update_registry_users(&transaction).await? {
            return Ok(TransactionStateModel::Pending(transaction));
        }
        

        Ok(TransactionStateModel::Sent(transaction))
    }

    async fn find_last(&self, registry: &RegistryModel) -> Result<Option<TransactionModel>, Box<dyn Error>> {
        let last_transaction: Option<TransactionModel> = self.transaction_repository.find_last(
            registry.id,
            registry.current_pack + 1,
        ).await?.map(|dto| dto.into()); 

        let last_transaction = match last_transaction {
            Some(transaction) => Some(transaction),
            None => self.transaction_repository.find_last(
                registry.id,
                registry.current_pack,
            ).await?.map(|dto| dto.into()),
        };

        Ok(last_transaction)
    }

    async fn ensure_complete(&self, registry: &RegistryModel, transaction: &TransactionModel) -> Result<bool, Box<dyn Error>> {
        if (registry.current_pack, registry.current_sequence) < (transaction.pack, transaction.sequence) {
            if !self.update_registry(registry, transaction).await? {
                return Ok(false);
            }
        }

        if !self.update_registry_users(transaction).await? {
            return Ok(false);
        }

        Ok(true)
    }

    async fn update_registry(&self, registry: &RegistryModel, transaction: &TransactionModel) -> Result<bool, Box<dyn Error>> {
        let update_dto = RegistryTransactionUpdateDto {
            id: registry.id,
            source_pack: registry.current_pack,
            target_pack: transaction.pack,
            source_updated_at: registry.updated_at,
            target_updated_at: transaction.created_at,
            source_sequence: registry.current_sequence,
            target_sequence: transaction.sequence,
        };

        self.registry_repository.update_transaction(&update_dto).await
    }

    async fn update_registry_users(
        &self,
        transaction: &TransactionModel,
    ) -> Result<bool, Box<dyn Error>> {
        let mut registry_users = self.registry_user_repository.list(
            transaction.registry_id, 
            &[transaction.source_user_id, transaction.target_user_id],
        ).await?;

        if !registry_users.iter().any(|dto| (dto.current_pack, dto.current_sequence) < (transaction.pack, transaction.sequence)) {
            return Ok(true);
        }

        let first_registry_user: RegistryUserModel = registry_users.pop().unwrap().into();
        let second_registry_user: RegistryUserModel = registry_users.pop().unwrap().into();

        let first_value = first_registry_user.balance.get(&transaction.currency).cloned();
        let first_value_base = first_value.clone().unwrap_or(BigDecimal::zero());
        let second_value = second_registry_user.balance.get(&transaction.currency).cloned();
        let second_value_base = second_value.clone().unwrap_or(BigDecimal::zero());

        let update_dtos = [
            RegistryUserUpdateDto {
                registry_id: transaction.registry_id,
                user_id: first_registry_user.user_id,
                updated_at: transaction.created_at,
                current_pack: transaction.pack,
                current_sequence: transaction.sequence,
                currency: transaction.currency.clone(),
                source_value: first_value,
                target_value: if first_registry_user.user_id == transaction.source_user_id {
                    &first_value_base - &transaction.amount
                }
                else {
                    &first_value_base + &transaction.amount
                }
            },
            RegistryUserUpdateDto {
                registry_id: transaction.registry_id,
                user_id: second_registry_user.user_id,
                updated_at: transaction.created_at,
                current_pack: transaction.pack,
                current_sequence: transaction.sequence,
                currency: transaction.currency.clone(),
                source_value: second_value,
                target_value: if second_registry_user.user_id == transaction.source_user_id {
                    &second_value_base - &transaction.amount
                }
                else {
                    &second_value_base + &transaction.amount
                }
            },
        ];

        self.registry_user_repository.update(&update_dtos).await
    }
}