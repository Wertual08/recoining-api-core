mod config;
mod logging;
mod grpc;
mod storage;
mod domain;
mod migrations;

use std::sync::Arc;
use std::time::Duration;

use config::Config;
use grpc::{AuthGrpcService, AuthServer};
use storage::RepositoryFactory;
use tonic::transport::Server;

use crate::domain::ServiceFactory;
use crate::grpc::{UsersGrpcService, UsersServer, RegistriesGrpcService, RegistriesServer, ProfileGrpcService, ProfileServer};
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

    print!("Initializing grpc services...");
    let auth = AuthGrpcService::new(
        Arc::clone(&logger),
        Arc::clone(&service_factory),
    );
    let users = UsersGrpcService::new(
        Arc::clone(&logger),
        Arc::clone(&service_factory),
    );
    let registries = RegistriesGrpcService::new(
        Arc::clone(&logger),
        Arc::clone(&service_factory),
    );
    let profile = ProfileGrpcService::new(
        Arc::clone(&logger),
        Arc::clone(&service_factory),
    );
    println!(" DONE");

    println!("Running server...");
    Server::builder()
        .http2_keepalive_interval(Some(Duration::from_secs(4)))
        .http2_keepalive_timeout(Some(Duration::from_secs(1)))
        .add_service(AuthServer::new(auth))
        .add_service(UsersServer::new(users))
        .add_service(RegistriesServer::new(registries))
        .add_service(ProfileServer::new(profile))
        .serve(config.server.host.parse().unwrap())
        .await?;

    println!("Exiting...");

    Ok(())
}
