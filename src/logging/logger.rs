use std::error::Error;

#[derive(Debug)]
pub struct Logger {

}

impl Logger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn log_fail(&self, error: &Box<dyn Error>) {
        eprintln!("{:?}", error);
    }
}