use std::error::Error;

use tonic::Status;

use crate::logging::Logger;

pub trait StatusResult<T> {
    fn consume_error(self, logger: &Logger) -> Result<T, Status>;
}

impl<T> StatusResult<T> for Result<T, Box<dyn Error>> {
    fn consume_error(self, logger: &Logger) -> Result<T, Status> {
        self.map_err(|err| {
            logger.log_fail(&err);
            Status::internal(err.to_string())
        })
    }
}