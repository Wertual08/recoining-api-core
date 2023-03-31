use std::{sync::Arc, error::Error, collections::HashMap};

use bigdecimal::BigDecimal;
use scylla::{prepared_statement::PreparedStatement, transport::errors::QueryError, QueryResult};
use tonic::async_trait;

use crate::storage::ScyllaContext;

use super::{UserDto, UserRepository};

#[derive(Debug)]
pub struct ScyllaUserRepository {
    scylla_context: Arc<ScyllaContext>,
    statement_insert: PreparedStatement,
    statement_find_id: PreparedStatement,
    statement_find_phone: PreparedStatement,
    statement_find_email: PreparedStatement,
}

impl ScyllaUserRepository {
    pub async fn new(scylla_context: Arc<ScyllaContext>) -> Result<Self, QueryError> {
        let statement_insert = scylla_context.session.prepare(format!("
            insert into {}.users (
                id,
                phone,
                email,
                login,
                image,
                balance
            ) values (?, ?, ?, ?, ?, ?)
            if not exists
        ", &scylla_context.keyspace)).await?;

        let statement_find_id = scylla_context.session.prepare(format!("
            select
                id,
                phone,
                email,
                login,
                image,
                balance
            from {}.users 
            where id = ?
        ", &scylla_context.keyspace)).await?;

        let statement_find_phone = scylla_context.session.prepare(format!("
            select
                id,
                phone,
                email,
                login,
                image,
                balance
            from {}.users 
            where phone = ?
        ", &scylla_context.keyspace)).await?;

        let statement_find_email = scylla_context.session.prepare(format!("
            select
                id,
                phone,
                email,
                login,
                image,
                balance
            from {}.users 
            where email = ?
        ", &scylla_context.keyspace)).await?;

        let result = Self {
            scylla_context,
            statement_insert,   
            statement_find_id,
            statement_find_phone,
            statement_find_email,
        };

        Ok(result)
    }
}

#[async_trait]
impl UserRepository for ScyllaUserRepository {
    async fn create(&self, dto: &UserDto) -> Result<bool, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_insert, (
            dto.id, 
            &dto.phone,
            &dto.email,
            &dto.login,
            &dto.image,
            &dto.balance,
        )).await?;

        Ok(result.single_row()?.columns[0].as_ref().unwrap().as_boolean().unwrap())
    }

    async fn find_id(&self, id: i64) -> Result<Option<UserDto>, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_find_id, (
            id, 
        )).await?;

        map_user_dto(result)
    }

    async fn find_phone(&self, phone: i64) -> Result<Option<UserDto>, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_find_phone, (
            phone, 
        )).await?;

        map_user_dto(result)
    }

    async fn find_email(&self, email: &String) -> Result<Option<UserDto>, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_find_email, (
            email, 
        )).await?;

        map_user_dto(result)
    }
}

fn map_user_dto(result: QueryResult) -> Result<Option<UserDto>, Box<dyn Error>> {
    let mapped = result.maybe_first_row_typed::<(
        i64, 
        i64, 
        String, 
        String, 
        String, 
        Option<HashMap<String, BigDecimal>>,
    )>()?.map(|row| {
        let (id, phone, email, login, image, balance) = row;
        UserDto { 
            id, 
            phone, 
            email, 
            login, 
            image,
            balance: balance.unwrap_or(HashMap::new()),
        }
    });

    Ok(mapped)
}