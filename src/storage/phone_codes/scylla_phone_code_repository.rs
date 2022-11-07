use std::{sync::Arc, error::Error};
use scylla::{prepared_statement::PreparedStatement, transport::errors::QueryError};
use tonic::async_trait;

use super::{super::ScyllaContext, PhoneCodeRepository, PhoneCodeDto};

#[derive(Debug)]
pub struct ScyllaPhoneCodeRepository {
    scylla_context: Arc<ScyllaContext>,
    statement_create: PreparedStatement,
    statement_delete: PreparedStatement,
    statement_find: PreparedStatement,
}

impl ScyllaPhoneCodeRepository {
    pub async fn new(scylla_context: Arc<ScyllaContext>) -> Result<Self, QueryError> {
        let statement_create = scylla_context.session.prepare(format!("
            insert into {}.phone_codes (
                phone,
                code,
                created_at,
                attempts
            ) values (?, ?, ?, ?)
            if not exists
            using ttl ?
        ", &scylla_context.keyspace)).await?;

        let statement_delete = scylla_context.session.prepare(format!("
            delete from {}.phone_codes
            where phone = ?
            if attempts = ?
        ", &scylla_context.keyspace)).await?;

        let statement_find = scylla_context.session.prepare(format!("
            select
                phone,
                code,
                created_at,
                attempts,
                ttl(code)
            from {}.phone_codes
            where phone = ?
        ", &scylla_context.keyspace)).await?;

        let result = Self {
            scylla_context,
            statement_create,
            statement_delete,
            statement_find,    
        };

        Ok(result)
    }
}

#[async_trait]
impl PhoneCodeRepository for ScyllaPhoneCodeRepository {
    async fn create(&self, dto: &PhoneCodeDto) -> Result<bool, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_create, (
            dto.phone, 
            dto.code,
            dto.created_at,
            dto.attempts,
            dto.ttl,
        )).await?;

        Ok(result.single_row()?.columns[0].as_ref().unwrap().as_boolean().unwrap())
    }

    async fn delete(&self, dto: &PhoneCodeDto) -> Result<bool, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_delete, (
            dto.phone,
            dto.attempts,
        )).await?;

        Ok(result.single_row()?.columns[0].as_ref().unwrap().as_boolean().unwrap())
    }

    async fn find(&self, phone: i64) -> Result<Option<PhoneCodeDto>, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_find, (
            phone, 
        )).await?;

        let mapped = result.maybe_first_row_typed::<(i64, i64, i64, i16, i32)>()?.map(|row| {
            let (phone, code, created_at, attempts, ttl) = row;
            PhoneCodeDto { 
                phone, 
                code, 
                attempts, 
                created_at,
                ttl,
            }
        });

        Ok(mapped)
    }
}