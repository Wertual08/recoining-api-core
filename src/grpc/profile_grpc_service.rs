pub mod api_profile {
    tonic::include_proto!("api_core.profile");
}

pub use api_profile::profile_server::ProfileServer;
use tonic::{Request, Response, Status};

use std::sync::Arc;

use crate::{domain::{ServiceFactory, registries::RegistryModel}, logging::Logger};

use self::api_profile::{profile_server::Profile, ListRegistriesResponse, ListRegistriesRequest, RegistryResource};

use super::{extensions::{StatusResult, AuthorizedRequest}};


#[derive(Debug)]
pub struct ProfileGrpcService {
    logger: Arc<Logger>,
    service_factory: Arc<ServiceFactory>,
}

impl ProfileGrpcService {
    pub fn new(
        logger: Arc<Logger>,
        service_factory: Arc<ServiceFactory>,
    ) -> Self {
        Self {
            logger,
            service_factory,
        }
    }
}

#[tonic::async_trait]
impl Profile for ProfileGrpcService {
    async fn list_registries(&self, request: Request<ListRegistriesRequest>) -> Result<Response<ListRegistriesResponse>, Status> {
        let request_data = request.get_ref();
        if request_data.limit <= 0 || request_data.limit > 64 {
            return Err(Status::invalid_argument("Limit must in [1:64]"));
        }

        let token = request.authorize(&self.logger, &self.service_factory.token())?;

        let registry_service = self.service_factory.registry();

        let registries = registry_service.list_user_registries(
            token.sub,
            request_data.last_updated_at,
            request_data.limit,
        ).await.consume_error(&self.logger)?;

        Ok(Response::new(ListRegistriesResponse { 
            registries: registries.into_iter().map(|model| RegistryResource::from(model)).collect()
        }))
    }
}

impl From<RegistryModel> for RegistryResource {
    fn from(model: RegistryModel) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            updated_at: model.updated_at,
            current_pack: model.current_pack,
            current_sequence: model.current_sequence as i32,
            variant: i16::from(model.variant) as i32,
            name: model.name,
            image: model.image,
        }
    }
}