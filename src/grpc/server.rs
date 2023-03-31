use std::{sync::Arc, time::Duration, net::SocketAddr, error::Error};

use tonic::transport::{Server, server::Router};

use crate::{domain::ServiceFactory, logging::Logger};

use super::{services::{
    AuthGrpcService, 
    UsersGrpcService, 
    RegistriesGrpcService, 
    ProfileGrpcService, AuthServer, UsersServer, RegistriesServer, ProfileServer, TransactionsGrpcService, TransactionsServer,
}, ServerConfig};

pub struct GrpcServer {
    address: SocketAddr,
    router: Router,
}

impl GrpcServer {
    pub fn new(
        config: &ServerConfig, 
        logger: &Arc<Logger>, 
        service_factory: &Arc<ServiceFactory>,
    ) -> Self {
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
        let transactions = TransactionsGrpcService::new(
            Arc::clone(&logger),
            Arc::clone(&service_factory),
        );

        Self {
            address: config.host.parse().unwrap(),
            router: Server::builder()
                .http2_keepalive_interval(Some(Duration::from_secs(4)))
                .http2_keepalive_timeout(Some(Duration::from_secs(1)))
                .add_service(AuthServer::new(auth))
                .add_service(UsersServer::new(users))
                .add_service(RegistriesServer::new(registries))
                .add_service(ProfileServer::new(profile))
                .add_service(TransactionsServer::new(transactions)),
        }
    }

    pub async fn serve(self) -> Result<(), Box<dyn Error>> {
        Ok(self.router.serve(self.address).await?)
    }
}