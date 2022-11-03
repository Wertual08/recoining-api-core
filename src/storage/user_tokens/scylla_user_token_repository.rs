use std::{sync::Arc, error::Error};

use scylla::{prepared_statement::PreparedStatement, transport::errors::QueryError, IntoTypedRows};
use tonic::async_trait;

use crate::storage::ScyllaContext;

use super::{UserTokenRepository, UserTokenDto};

#[derive(Debug)]
pub struct ScyllaUserTokenRepository {
    scylla_context: Arc<ScyllaContext>,
    statement_create: PreparedStatement,
    statement_exists: PreparedStatement,
}

impl ScyllaUserTokenRepository {
    pub async fn new(scylla_context: Arc<ScyllaContext>) -> Result<Self, QueryError> {
        let statement_create = scylla_context.session.prepare(format!("
            insert into {}.user_tokens (
                user_id,
                id
            ) values (?, ?)
            using ttl ?
        ", &scylla_context.keyspace)).await?;

        let statement_exists = scylla_context.session.prepare(format!("
            select count(*)
            from {}.user_tokens
            where user_id = ?
            and id = ?
        ", &scylla_context.keyspace)).await?;

        let result = Self {
            scylla_context,
            statement_create,
            statement_exists, 
        };

        Ok(result)
    }
}

#[async_trait]
impl UserTokenRepository for ScyllaUserTokenRepository {
    async fn create(&self, dto: &UserTokenDto, ttl: i32) -> Result<(), Box<dyn Error>> {
        self.scylla_context.session.execute(&self.statement_create, (
            dto.user_id, 
            &dto.id,
            ttl,
        )).await?;

        Ok(())
    }

    async fn exists(&self, dto: &UserTokenDto) -> Result<bool, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_exists, (
            dto.user_id,
            &dto.id, 
        )).await?;

        let rows = result.rows.unwrap(); 
        let row = rows.into_typed::<(i32,)>().next().unwrap();
        let (success, ) = row?;
        Ok(success > 0)
    }
}