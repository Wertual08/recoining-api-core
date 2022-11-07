use std::{sync::Arc, error::Error};
use bigdecimal::BigDecimal;
use scylla::{prepared_statement::PreparedStatement, transport::errors::QueryError, IntoTypedRows};
use tonic::async_trait;

use super::{super::ScyllaContext, TransactionRepository, TransactionDto};

#[derive(Debug)]
pub struct ScyllaTransactionRepository {
    scylla_context: Arc<ScyllaContext>,
    statement_create: PreparedStatement,
    statement_list: PreparedStatement,
}

impl ScyllaTransactionRepository {
    pub async fn new(scylla_context: Arc<ScyllaContext>) -> Result<Self, QueryError> {
        let statement_create = scylla_context.session.prepare(format!("
            insert into {}.transactions (
                registry_id,
                pack,
                sequence,
                created_at,
                source_user_id,
                target_user_id,
                variant,
                amount,
                currency,
                label,
                description,
                hash
            ) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            if not exists
        ", &scylla_context.keyspace)).await?;

        let statement_list = scylla_context.session.prepare(format!("
            select
                registry_id,
                pack,
                sequence,
                created_at,
                source_user_id,
                target_user_id,
                variant,
                amount,
                currency,
                label,
                description,
                hash
            from {}.transactions
            where registry_id = ?
            and pack = ?
            and sequence <= ?
            order by sequence desc
            limit ?
        ", &scylla_context.keyspace)).await?;

        let result = Self {
            scylla_context,
            statement_create,
            statement_list,    
        };

        Ok(result)
    }
}

#[async_trait]
impl TransactionRepository for ScyllaTransactionRepository {
    async fn create(&self, dto: &TransactionDto) -> Result<bool, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_create, (
            dto.registry_id,
            dto.pack,
            dto.sequence,
            dto.created_at,
            dto.source_user_id,
            dto.target_user_id,
            dto.variant,
            &dto.amount,
            &dto.currency,
            &dto.label,
            &dto.description,
            &dto.hash,
        )).await?;

        Ok(result.single_row()?.columns[0].as_ref().unwrap().as_boolean().unwrap())
    }

    async fn list(&self, registry_id: i64, pack: i64, last_sequence: i16, limit: i32) -> Result<Vec<TransactionDto>, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_list, (
            registry_id, 
            pack,
            last_sequence,
            limit,
        )).await?;

        if let Some(rows) = result.rows {
            let mut mapped = Vec::new();

            for row in rows.into_typed::<(
                i64,
                i64,
                i16,
                i64,
                i64,
                i64,
                i16,
                BigDecimal,
                String,
                String,
                String,
                Vec<u8>,
            )>() {
                let (
                    registry_id, 
                    pack, 
                    sequence, 
                    created_at, 
                    source_user_id, 
                    target_user_id, 
                    variant, 
                    amount, 
                    currency, 
                    label, 
                    description, 
                    hash,
                ) = row?; 

                let dto = TransactionDto { 
                    registry_id, 
                    pack, 
                    sequence, 
                    created_at, 
                    source_user_id, 
                    target_user_id, 
                    variant, 
                    amount, 
                    currency, 
                    label, 
                    description, 
                    hash,
                };

                mapped.push(dto);
            }

            return Ok(mapped);
        }

        Ok(Vec::new())
    }
}