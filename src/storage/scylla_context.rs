use scylla::{Session, SessionBuilder, transport::errors::NewSessionError};

use super::ScyllaConfig;

#[derive(Debug)]
pub struct ScyllaContext {
    pub session: Session,
    pub keyspace: String,
}

impl ScyllaContext {
    pub async fn new(config: &ScyllaConfig) -> Result<Self, NewSessionError> {
        let session = SessionBuilder::new()
            .known_nodes(&config.hosts)
            .build()
            .await?;
        
        let context = Self {
            session: session,
            keyspace: config.keyspace.clone(),
        };
        
        Ok(context)
    }
}