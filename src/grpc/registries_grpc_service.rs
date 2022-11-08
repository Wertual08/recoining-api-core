pub mod api_registries {
    tonic::include_proto!("api_core.registries");
}

pub use api_registries::registries_server::RegistriesServer;
use tonic::{Request, Response, Status};

use std::sync::Arc;

use crate::{domain::{ServiceFactory, registries::RegistryModel}, logging::Logger};

use self::api_registries::{CreateDirectRequest, CreateResponse, registries_server::Registries, FindRequest, FindResponse, create_response::{Payload, Retry}, RegistryResource};

use super::extensions::{AuthorizedRequest, StatusResult};

#[derive(Debug)]
pub struct RegistriesGrpcService {
    logger: Arc<Logger>,
    service_factory: Arc<ServiceFactory>,
}

impl RegistriesGrpcService {
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
impl Registries for RegistriesGrpcService {
    async fn create_direct(&self, request: Request<CreateDirectRequest>) -> Result<Response<CreateResponse>, Status> {
        let access_token = request.authorize(&self.logger, &self.service_factory.token())?;
        let request_data = request.get_ref();

        let registry_service = self.service_factory.registry();

        let model_option = registry_service.create_direct(
            access_token.sub, 
            request_data.user_id, 
            request_data.name.clone(), 
            request_data.image.clone(),
        ).await.consume_error(&self.logger)?;

        let payload = match model_option {
            Some(model) => {
                Payload::Registry(model.into())
            }
            None => Payload::Retry(Retry {})
        };

        Ok(Response::new(CreateResponse {
            payload: Some(payload),
        }))
    }

    async fn find(&self, request: Request<FindRequest>) -> Result<Response<FindResponse>, Status> {
        let access_token = request.authorize(&self.logger, &self.service_factory.token())?;
        let request_data = request.get_ref();
        let registry_id = request_data.id;
        let user_id = access_token.sub;

        let registry_service = self.service_factory.registry();

        if !registry_service.access(registry_id, &[user_id]).await.consume_error(&self.logger)? {
            return Err(Status::permission_denied("Access denied to registry"));
        }

        let registry_option = registry_service.find(registry_id).await.consume_error(&self.logger)?;

        if let Some(model) = registry_option {

        }
        else {
            return Err(Status::not_found("Registry not found"));
        }
        let registry_option = registry_service.find(registry_id).await.consume_error(&self.logger)?;

        if let Some(model) = registry_option {
            Ok(Response::new(FindResponse {
                registry: Some(model.into()),
            }))
        }
        else {
            Err(Status::not_found("Registry not found"))
        }
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