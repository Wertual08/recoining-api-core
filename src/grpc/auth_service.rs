pub mod api_auth {
    tonic::include_proto!("api.auth");
}

use std::{sync::Arc, error::Error};

pub use api_auth::auth_server::{Auth, AuthServer};
use api_auth::{
    SignInPhoneRequest, 
    SignInPhoneResponse, 
    SignInResultResource, 
    sign_in_result_resource::{
        Payload,
        Success,
    }
};
use tonic::{Request, Response, Status};

use crate::domain::{ServiceFactory, codes::{CodeSendModel, CodeAttemptModel}};

use self::api_auth::{SendCodePhoneResponse, SendCodePhoneRequest, SendCodeResultResource, send_code_result_resource, sign_in_result_resource::{Fail, Absent, Retry}};

#[derive(Debug)]
pub struct AuthService {
    service_factory: Arc<ServiceFactory>,
}

impl AuthService {
    pub fn new(service_factory: Arc<ServiceFactory>) -> Self {
        Self {
            service_factory,
        }
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn send_code_phone(&self, request: Request<SendCodePhoneRequest>) -> Result<Response<SendCodePhoneResponse>, Status> {
        let request_data = request.get_ref();
        
        let service = self.service_factory.code();
        
        let result = service
            .send_phone(request_data.phone)
            .await
            .status_result()?;

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
        ).await.status_result()?;

        let payload = match attempt_result {
            CodeAttemptModel::Success => {
                let user_service = self.service_factory.user();

                let id_option = user_service.get_id_phone(request_data.phone).await.status_result()?;

                if let Some(id) = id_option {
                    let token_service = self.service_factory.token();

                    let (refresh_token, refresh_expires_at) = token_service
                        .create_refresh(id)
                        .await
                        .status_result()?;

                    let (access_token, access_expires_at) = token_service
                        .create_access(id)
                        .await
                        .status_result()?;

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
}

trait Statusable<T> {
    fn status_result(self) -> Result<T, Status>;
}

impl<T> Statusable<T> for Result<T, Box<dyn Error>> {
    fn status_result(self) -> Result<T, Status> {
        self.map_err(|err| {
            eprintln!("{:?}", err);
            Status::internal(err.to_string())
        })
    }
}