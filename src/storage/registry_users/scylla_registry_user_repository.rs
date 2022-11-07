use std::{sync::Arc, error::Error, collections::HashMap};
use bigdecimal::BigDecimal;
use scylla::{prepared_statement::PreparedStatement, transport::errors::QueryError, batch::{Batch, BatchType}, IntoTypedRows};
use tonic::async_trait;

use super::{super::ScyllaContext, RegistryUserRepository, RegistryUserDto, RegistryUserUpdateDto};

#[derive(Debug)]
pub struct ScyllaRegistryUserRepository {
    scylla_context: Arc<ScyllaContext>,
    statement_create: PreparedStatement,
    statement_update: PreparedStatement,
    statement_list: PreparedStatement,
}

impl ScyllaRegistryUserRepository {
    pub async fn new(scylla_context: Arc<ScyllaContext>) -> Result<Self, QueryError> {
        let statement_create = scylla_context.session.prepare(format!("
            insert into {}.registry_users (
                registry_id,
                user_id,
                updated_at,
                current_pack,
                current_sequence,
                balance
            ) values (?, ?, ?, ?, ?, ?)
            if not exists
        ", &scylla_context.keyspace)).await?;

        let statement_update = scylla_context.session.prepare(format!("
            update {}.registry_users
            set
                updated_at = ?,
                current_pack = ?,
                current_sequence = ?,
                balance[?] = ?
            where registry_id = ?
            and user_id = ?
            if balance[?] = ?;
        ", &scylla_context.keyspace)).await?;

        let statement_list = scylla_context.session.prepare(format!("
            select
                registry_id,
                user_id,
                updated_at,
                current_pack,
                current_sequence,
                balance
            from {}.registry_users
            where registry_id = ?
            and user_id in ?
        ", &scylla_context.keyspace)).await?;

        let result = Self {
            scylla_context,
            statement_create,
            statement_update,
            statement_list,    
        };

        Ok(result)
    }
}

#[async_trait]
impl RegistryUserRepository for ScyllaRegistryUserRepository {
    async fn create(&self, dtos: &[RegistryUserDto]) -> Result<bool, Box<dyn Error>> {
        let mut batch = Batch::new(BatchType::Unlogged);
        let mut args = Vec::with_capacity(dtos.len());

        for dto in dtos {
            batch.append_statement(self.statement_create.clone());
            args.push((
                dto.updated_at,
                dto.registry_id,
                dto.user_id,
                dto.current_pack,
                dto.current_sequence,
                &dto.balance,
            ));
        }
        
        let result = self.scylla_context.session.batch(&batch, args).await?;

        Ok(result.first_row()?.columns[0].as_ref().unwrap().as_boolean().unwrap())
    }

    async fn update(&self, dtos: &[RegistryUserUpdateDto]) -> Result<bool, Box<dyn Error>> {
        let mut batch = Batch::new(BatchType::Unlogged);
        let mut args = Vec::with_capacity(dtos.len());

        for dto in dtos {
            batch.append_statement(self.statement_update.clone());
            args.push((
                dto.updated_at,
                dto.current_pack,
                dto.current_sequence,
                &dto.target_value,
                dto.registry_id,
                dto.user_id,
                &dto.source_value,
            ));
        }
        
        let result = self.scylla_context.session.batch(&batch, args).await?;

        Ok(result.first_row()?.columns[0].as_ref().unwrap().as_boolean().unwrap())
    }

    async fn list(&self, registry_id: i64, user_ids: &[i64]) -> Result<Vec<RegistryUserDto>, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_list, (
            registry_id,
            user_ids, 
        )).await?;

        if let Some(rows) = result.rows {
            let mut mapped = Vec::new();

            for row in rows.into_typed::<(i64, i64, i64, i64, i16, Option<HashMap<String, BigDecimal>>)>() {
                let (
                    registry_id,
                    user_id,
                    updated_at,
                    current_pack,
                    current_sequence,
                    balance,
                ) = row?; 

                mapped.push(
                    RegistryUserDto {
                        registry_id,
                        user_id,
                        updated_at,
                        current_pack,
                        current_sequence,
                        balance: balance.unwrap_or(HashMap::new()),
                    }
                );
            }

            return Ok(mapped);
        }

        Ok(Vec::new())
    }
}