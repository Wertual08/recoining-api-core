use std::{sync::Arc, error::Error};
use scylla::{prepared_statement::PreparedStatement, transport::errors::QueryError, IntoTypedRows};
use tonic::async_trait;

use crate::storage::ScyllaContext;

use super::{UserRegistryDto, UserRegistryRepository};

#[derive(Debug)]
pub struct ScyllaUserRegistryRepository {
    scylla_context: Arc<ScyllaContext>,
    statement_list: PreparedStatement,
}

impl ScyllaUserRegistryRepository {
    pub async fn new(scylla_context: Arc<ScyllaContext>) -> Result<Self, QueryError> {
        let statement_list = scylla_context.session.prepare(format!("
            select
                user_id,
                updated_at,
                registry_id
            from {}.user_registries
            where user_id = ?
            and updated_at <= ?
            order by updated_at desc
            limit ?;
        ", &scylla_context.keyspace)).await?;

        let result = Self {
            scylla_context,
            statement_list,    
        };

        Ok(result)
    }
}

#[async_trait]
impl UserRegistryRepository for ScyllaUserRegistryRepository {
    async fn list(&self, user_id: i64, last_updated_at: i64, limit: i32) -> Result<Vec<UserRegistryDto>, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_list, (
            user_id,
            last_updated_at, 
            limit, 
        )).await?;

        if let Some(rows) = result.rows {
            let mut mapped = Vec::new();

            for row in rows.into_typed::<(i64, i64, i64)>() {
                let (
                    user_id,
                    updated_at,
                    registry_id,
                ) = row?; 

                let dto = UserRegistryDto {
                    user_id,
                    updated_at,
                    registry_id,
                };

                mapped.push(dto);
            }

            return Ok(mapped);
        }

        Ok(Vec::new())
    }
}