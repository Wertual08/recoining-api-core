use std::{sync::Arc, error::Error, collections::HashMap};

use bigdecimal::BigDecimal;
use scylla::{prepared_statement::PreparedStatement, transport::errors::QueryError, IntoTypedRows};
use tonic::async_trait;

use crate::storage::ScyllaContext;

use super::{UserDto, UserRepository};

#[derive(Debug)]
pub struct ScyllaUserRepository {
    scylla_context: Arc<ScyllaContext>,
    statement_insert: PreparedStatement,
    statement_find_phone: PreparedStatement,
}

impl ScyllaUserRepository {
    pub async fn new(scylla_context: Arc<ScyllaContext>) -> Result<Self, QueryError> {
        let statement_insert = scylla_context.session.prepare(format!("
            insert into {}.users (
                id,
                phone,
                email,
                login,
                balance
            ) values (?, ?, ?, ?, ?)
            if not exists
        ", &scylla_context.keyspace)).await?;

        let statement_find_phone = scylla_context.session.prepare(format!("
            select
                id,
                phone,
                email,
                login,
                balance
            from {}.users 
            where phone = ?
        ", &scylla_context.keyspace)).await?;

        let result = Self {
            scylla_context,
            statement_insert,   
            statement_find_phone,
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
            &dto.balance,
        )).await?;

        let rows = result.rows.unwrap(); 
        let row = rows.into_typed::<(
            bool, 
            Option<i64>, 
            Option<HashMap<String, BigDecimal>>, 
            Option<String>, 
            Option<String>, 
            Option<i64>,
        )>().next().unwrap();
        let (success, _, _, _, _, _) = row?;
        Ok(success)
    }

    async fn find_phone(&self, phone: i64) -> Result<Option<UserDto>, Box<dyn Error>> {
        let result = self.scylla_context.session.execute(&self.statement_find_phone, (
            phone, 
        )).await?;

        if let Some(rows) = result.rows {
            if let Some(row) = rows.into_typed::<(i64, i64, String, String, Option<HashMap<String, BigDecimal>>)>().next() {
                let (id, phone, email, login, balance) = row?;
                return Ok(Some(UserDto { 
                    id, 
                    phone, 
                    email, 
                    login, 
                    balance: balance.unwrap_or(HashMap::new()),
                }))
            }
        }

        Ok(None)
    }
}