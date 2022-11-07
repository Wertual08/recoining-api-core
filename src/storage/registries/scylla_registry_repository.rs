use std::{sync::Arc, error::Error};
use scylla::{prepared_statement::PreparedStatement, transport::errors::QueryError};
use tonic::async_trait;

use super::{super::ScyllaContext, RegistryRepository, RegistryDto};

#[derive(Debug)]
pub struct ScyllaRegistryRepository {
    scylla_context: Arc<ScyllaContext>,
    statement_create: PreparedStatement,
    statement_find: PreparedStatement,
}

impl ScyllaRegistryRepository {
    pub async fn new(scylla_context: Arc<ScyllaContext>) -> Result<Self, QueryError> {
        let statement_create = scylla_context.session.prepare(format!("
            insert into {}.registries (
                id,
                current_pack,
                current_sequence,
                variant,
                created_at,
                updated_at,
                name,
                image
            ) values (?, ?, ?, ?, ?, ?, ?, ?)
            if not exists
        ", &scylla_context.keyspace)).await?;

        let statement_find = scylla_context.session.prepare(format!("
            select
                id,
                current_pack,
                current_sequence,
                variant,
                created_at,
                updated_at,
                name,
                image
            from {}.registries
            where id = ?
        ", &scylla_context.keyspace)).await?;

        let result = Self {
            scylla_context,
            statement_create,
            statement_find,    
        };

        Ok(result)
    }
}

#[async_trait]
impl RegistryRepository for ScyllaRegistryRepository {
    async fn create(&self, dto: &RegistryDto) -> Result<bool, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_create, (
            dto.id,
            dto.variant,
            dto.created_at,
            dto.updated_at,
            &dto.name,
            &dto.image,
        )).await?;

        Ok(result.single_row()?.columns[0].as_ref().unwrap().as_boolean().unwrap())
    }

    async fn find(&self, id: i64) -> Result<Option<RegistryDto>, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_find, (
            id, 
        )).await?;

        let mapped = result.maybe_first_row_typed::<(i64, i64, i16, i16, i64, i64, String, String)>()?.map(|row| {
            let (id, current_pack, current_sequence, variant, created_at, updated_at, name, image) = row;
            RegistryDto { 
                id, 
                current_pack,
                current_sequence,
                variant, 
                created_at, 
                updated_at, 
                name, 
                image,
            }
        });

        Ok(mapped)
    }
}