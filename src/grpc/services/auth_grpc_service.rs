pub mod api_auth {
    tonic::include_proto!("api_core.auth");
}

pub use api_auth::auth_server::AuthServer;

use std::sync::Arc;
use api_auth::{
    auth_server::Auth, 
    SignInPhoneRequest, 
    SignInPhoneResponse, 
    SignInResultResource, 
    sign_in_result_resource::{
        Payload,
        Success,
    }
};
use tonic::{Request, Response, Status};

use crate::{domain::{ServiceFactory, codes::{CodeSendModel, CodeAttemptModel}}, logging::Logger};

use self::api_auth::{
    SendCodePhoneResponse, 
    SendCodePhoneRequest, 
    SendCodeResultResource, 
    send_code_result_resource, 
    sign_in_result_resource::{Fail, Absent, Retry}, 
    CreateGenericAccessTokenRequest, 
    CreateGenericAccessTokenResponse,
};

use super::extensions::StatusResult;

#[derive(Debug)]
pub struct AuthGrpcService {
    logger: Arc<Logger>,
    service_factory: Arc<ServiceFactory>,
}

impl AuthGrpcService {
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
impl Auth for AuthGrpcService {
    async fn send_code_phone(&self, request: Request<SendCodePhoneRequest>) -> Result<Response<SendCodePhoneResponse>, Status> {
        let request_data = request.get_ref();
        
        let service = self.service_factory.code();
        
        let result = service
            .send_phone(request_data.phone)
            .await
            .consume_error(&self.logger)?;

        let payload = match result {
            CodeSendModel::Success(timeout, valid) => send_code_result_resource::Payload::Success(
                send_code_result_resource::Success {
                    timeout_until: timeout,
                    valid_until: valid,
                }
            ),
            CodeSendModel::Timeout(timeout) => send_code_result_resource::Payload::Timeout(
                send_code_result_resource::Timeout {
                    timeout_until: timeout,
                },
            ),
            CodeSendModel::Fail => send_code_result_resource::Payload::Fail(
                send_code_result_resource::Fail {
                }
            ),
            CodeSendModel::Retry => send_code_result_resource::Payload::Retry(
                send_code_result_resource::Retry {
                }
            )
        };

        Ok(
            Response::new(
                SendCodePhoneResponse { 
                    result: Some(SendCodeResultResource { payload: Some(payload) })
                }
            )
        )
    }


    async fn sign_in_phone(&self, request: Request<SignInPhoneRequest>) -> Result<Response<SignInPhoneResponse>, Status> {
        let request_data = request.get_ref();
        
        let code_service = self.service_factory.code();

        let attempt_result = code_service.attempt_phone(
            request_data.phone, 
            request_data.code,
        ).await.consume_error(&self.logger)?;

        let payload = match attempt_result {
            CodeAttemptModel::Success => {
                let user_service = self.service_factory.user();

                let id_option = user_service
                    .get_id_phone(request_data.phone)
                    .await
                    .consume_error(&self.logger)?;

                if let Some(id) = id_option {
                    let token_service = self.service_factory.token();

                    let (refresh_token, refresh_expires_at) = token_service
                        .create_refresh(id)
                        .await
                        .consume_error(&self.logger)?;

                    let (access_token, access_expires_at) = token_service
                        .create_access(id)
                        .consume_error(&self.logger)?;

                    Payload::Success(Success {
                        user_id: id,
                        refresh_token: refresh_token,
                        refresh_expires_at: refresh_expires_at,
                        access_token: access_token,
                        access_expires_at: access_expires_at,
                    })
                }
                else {
                    Payload::Retry(Retry {
                    })
                }
            }
            CodeAttemptModel::Absent => Payload::Absent(Absent {}),
            CodeAttemptModel::Fail(attempts) => Payload::Fail(
                Fail { attempts_left: attempts as i32 },
            ),
            CodeAttemptModel::Retry => Payload::Retry(
                Retry {},
            ),
        };
        
        Ok(
            Response::new(
                SignInPhoneResponse { 
                    result: Some(SignInResultResource { payload: Some(payload) }),
                }
            )
        )
    }

    async fn create_generic_access_token(
        &self, 
        request: Request<CreateGenericAccessTokenRequest>
    ) -> Result<Response<CreateGenericAccessTokenResponse>, Status> {
        if let Some(auth_metadata) = request.metadata().get("authorization") {
            if let Ok(str) = auth_metadata.to_str() {
                if let Some(token) = str.strip_prefix("refresh ") {
                    let token_service = self.service_factory.token();
                    
                    if let Some(user_id) = token_service.find_refresh(token).await.consume_error(&self.logger)? {
                        let (token, expires_at) = token_service.create_access(user_id).consume_error(&self.logger)?;

                        Ok(
                            Response::new(
                                CreateGenericAccessTokenResponse { 
                                    token,
                                    expires_at,
                                }
                            )
                        )
                    }
                    else {
                        Err(Status::unauthenticated("Token does not exists"))
                    }
                }
                else {
                    Err(Status::unauthenticated("Invalid token type"))
                }
            }
            else {
                Err(Status::unauthenticated("Invalid authorization header format"))
            }
        }
        else {
            Err(Status::unauthenticated("No authorization was present"))
        }
    }
}