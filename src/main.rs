mod config;
mod grpc;
mod storage;
mod domain;
mod migrations;

use std::sync::Arc;

use config::Config;
use grpc::auth_service::{AuthService, AuthServer};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use serde::{Serialize, Deserialize};
use storage::RepositoryFactory;
use tonic::transport::Server;
use uuid::Uuid;

use crate::domain::ServiceFactory;
use crate::storage::ScyllaContext;
use crate::migrations::migrate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Reading config...");
    let config = Config::new();
    println!("Starting with config: {}", config.serialize());

    println!("Connectiong to database...");
    let scylla_context = Arc::new(ScyllaContext::new(&config.scylla).await?);
    println!("Database connection established");

    migrate(&scylla_context, &String::from("migrations")).await?;

    println!("Initializing repositories...");
    let repository_factory = RepositoryFactory::new(&scylla_context).await?;
    println!("Repositories initialization complete");

    println!("Initializing services...");
    let service_factory = Arc::new(ServiceFactory::new(repository_factory));
    println!("Services initialization complete");

    let auth = AuthService::new(Arc::clone(&service_factory));

    println!("Running server...");

    Server::builder()
        .add_service(AuthServer::new(auth))
        .serve(config.server.host.parse().unwrap())
        .await?;

    println!("Server is shut down");

    Ok(())
}
