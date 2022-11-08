use tonic::{Request, Status};

use crate::{domain::tokens::{TokenService, AccessTokenModel}, logging::Logger};

use super::StatusResult;

pub trait AuthorizedRequest {
    fn authorize(&self, logger: &Logger, token_service: &TokenService) -> Result<AccessTokenModel, Status>;
}

impl<T> AuthorizedRequest for Request<T> {
    fn authorize(&self, logger: &Logger, token_service: &TokenService) -> Result<AccessTokenModel, Status> {
        match self.metadata().get("authorization") {
            Some(metadata) => match metadata.to_str() {
                Ok(str) => match str.strip_prefix("access ") {
                    Some(token) => match token_service.decode_access(token).consume_error(&logger)? {
                        Some(model) => Ok(model),
                        None => Err(Status::unauthenticated("Invalid access token")),
                    },
                    None => Err(Status::unauthenticated("Invalid token type")),
                },
                Err(_) => Err(Status::unauthenticated("Invalid authorization header format")),
            },
            None => Err(Status::unauthenticated("No authorization was present")),
        }
    }
}