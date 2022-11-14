mod config;
mod logging;
mod grpc;
mod storage;
mod domain;
mod migrations;

use std::sync::Arc;

use config::Config;
use storage::RepositoryFactory;

use crate::domain::ServiceFactory;
use crate::grpc::GrpcServer;
use crate::logging::Logger;
use crate::storage::ScyllaContext;
use crate::migrations::migrate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Reading config...");
    let config = Config::new()?;
    println!(" DONE: {}", config.serialize());

    print!("Connecting to database...");
    let scylla_context = Arc::new(ScyllaContext::new(&config.scylla).await?);
    println!(" DONE");

    migrate(&scylla_context, &String::from("migrations")).await?;

    print!("Initializing logger...");
    let logger = Arc::new(Logger::new());
    println!(" DONE");

    print!("Initializing repositories...");
    let repository_factory = RepositoryFactory::new(&scylla_context).await?;
    println!(" DONE");

    print!("Initializing services...");
    let service_factory = Arc::new(ServiceFactory::new(config.services, repository_factory)?);
    println!(" DONE");

    print!("Initializing grpc server...");
    let grpc_server = GrpcServer::new(
        &config.server,
        &logger,
        &service_factory,
    );
    println!(" DONE");

    println!("Running server...");
    grpc_server.serve().await?;
    println!("Exiting...");

    Ok(())
}
