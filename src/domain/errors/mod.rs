use std::error::Error;
use std::fmt;

pub type DomainError = Box<dyn Error>;

#[derive(Debug, Clone)]
pub struct UnknownError {}

impl UnknownError {
    pub fn new() -> DomainError {
        Box::new(UnknownError {})
    }
}

impl fmt::Display for UnknownError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown error")
    }
}

impl Error for UnknownError {}
