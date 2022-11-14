use std::{sync::Arc, error::Error};
use scylla::{prepared_statement::PreparedStatement, transport::errors::QueryError, IntoTypedRows};
use tonic::async_trait;

use super::{super::ScyllaContext, RegistryRepository, RegistryDto, RegistryTransactionUpdateDto};

#[derive(Debug)]
pub struct ScyllaRegistryRepository {
    scylla_context: Arc<ScyllaContext>,
    statement_create: PreparedStatement,
    statement_update: PreparedStatement,
    statement_find: PreparedStatement,
    statement_list: PreparedStatement,
}

impl ScyllaRegistryRepository {
    pub async fn new(scylla_context: Arc<ScyllaContext>) -> Result<Self, QueryError> {
        let select_base = format!("
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
        ", &scylla_context.keyspace);

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

        let statement_update = scylla_context.session.prepare(format!("
            update {}.registries
            set
                current_pack = ?,
                current_sequence = ?,
                updated_at = ?
            where id = ?
            if current_pack = ?
            and current_sequence = ?
            and updated_at = ?
        ", &scylla_context.keyspace)).await?;

        let statement_find = scylla_context.session.prepare(format!("
            {}
            where id = ?
        ", &select_base)).await?;

        let statement_list = scylla_context.session.prepare(format!("
            {}
            where id in ?
        ", &select_base)).await?;

        let result = Self {
            scylla_context,
            statement_create,
            statement_update,
            statement_find,  
            statement_list,  
        };

        Ok(result)
    }
}

#[async_trait]
impl RegistryRepository for ScyllaRegistryRepository {
    async fn create(&self, dto: &RegistryDto) -> Result<bool, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_create, (
            dto.id,
            dto.current_pack,
            dto.current_sequence,
            dto.variant,
            dto.created_at,
            dto.updated_at,
            &dto.name,
            &dto.image,
        )).await?;

        Ok(result.single_row()?.columns[0].as_ref().unwrap().as_boolean().unwrap())
    }
    
    async fn update_transaction(&self, dto: &RegistryTransactionUpdateDto) -> Result<bool, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_update, (
            dto.target_pack,
            dto.target_sequence,
            dto.target_updated_at,
            dto.id,
            dto.source_pack,
            dto.source_sequence,
            dto.source_updated_at,
        )).await?;

        Ok(result.single_row()?.columns[0].as_ref().unwrap().as_boolean().unwrap())
    }

    async fn find(&self, id: i64) -> Result<Option<RegistryDto>, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_find, (
            id, 
        )).await?;

        let mapped = result.maybe_first_row_typed::<RowType>()?.map(|row| {
            RegistryDto::from(row)
        });

        Ok(mapped)
    }

    async fn list(&self, ids: &[i64]) -> Result<Vec<RegistryDto>, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_list, (
            ids, 
        )).await?;

        if let Some(rows) = result.rows {
            let mut dtos = Vec::new();

            for row in rows.into_typed::<RowType>() {
                dtos.push(RegistryDto::from(row?));
            }

            Ok(dtos)
        }
        else {
            Ok(Vec::new())
        }
    }
}

type RowType = (i64, i64, i16, i16, i64, i64, String, String);

impl From<(i64, i64, i16, i16, i64, i64, String, String)> for RegistryDto {
    fn from(row: (i64, i64, i16, i16, i64, i64, String, String)) -> Self {
        let (id, current_pack, current_sequence, variant, created_at, updated_at, name, image) = row;
        Self { 
            id, 
            current_pack,
            current_sequence,
            variant, 
            created_at, 
            updated_at, 
            name, 
            image,
        }
    }
}