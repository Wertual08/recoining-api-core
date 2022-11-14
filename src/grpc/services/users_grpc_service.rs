pub mod api_users {
    tonic::include_proto!("api_core.users");
}

pub use api_users::users_server::UsersServer;
use bigdecimal::ToPrimitive;
use tonic::{Request, Response, Status};

use std::{sync::Arc, collections::HashMap};

use crate::{domain::{ServiceFactory, users::UserModel}, logging::Logger};

use self::api_users::{users_server::Users, FindIdRequest, FindResponse, FindPhoneRequest, FindEmailRequest, UserResource};

use super::extensions::StatusResult;


#[derive(Debug)]
pub struct UsersGrpcService {
    logger: Arc<Logger>,
    service_factory: Arc<ServiceFactory>,
}

impl UsersGrpcService {
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
impl Users for UsersGrpcService {
    async fn find_id(&self, request: Request<FindIdRequest>) -> Result<Response<FindResponse>, Status> {
        let request_data = request.get_ref();

        let user_service = self.service_factory.user();

        let model_option = user_service.find_id(request_data.id).await.consume_error(&self.logger)?;

        Ok(Response::new(FindResponse { 
            user: model_option.map(|model| UserResource::from(model))
        }))
    }
    
    async fn find_phone(&self, request: Request<FindPhoneRequest>) -> Result<Response<FindResponse>, Status> {
        let request_data = request.get_ref();

        let user_service = self.service_factory.user();

        let model_option = user_service.find_phone(request_data.phone).await.consume_error(&self.logger)?;

        Ok(Response::new(FindResponse { 
            user: model_option.map(|model| UserResource::from(model))
        }))
    }
    
    async fn find_email(&self, request: Request<FindEmailRequest>) -> Result<Response<FindResponse>, Status> {
        let request_data = request.get_ref();

        let user_service = self.service_factory.user();

        let model_option = user_service.find_email(&request_data.email).await.consume_error(&self.logger)?;

        Ok(Response::new(FindResponse { 
            user: model_option.map(|model| UserResource::from(model))
        }))
    }
}

impl From<UserModel> for UserResource {
    fn from(model: UserModel) -> Self {
        let balance = HashMap::from_iter(
            model.balance
                .into_iter()
                .map(|(key, value)| (key, value.to_f64().unwrap()))
        );

        Self {
            id: model.id,
            phone: model.phone,
            email: model.email,
            login: model.login,
            image: model.image,
            balance,
        }
    }
}