use std::error::*;
use std::fmt;
use std::result;

pub type ServiceResult<T> = result::Result<T, ServiceError>;

#[derive(Debug, PartialEq)]
pub enum ServiceError {
    AccountAlreadyExists,
    AuthorizeFailed,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServiceError::AccountAlreadyExists => f.write_str("AccountAlreadyExists"),
            ServiceError::AuthorizeFailed => f.write_str("AuthorizeFailed"),
        }
    }
}

impl Error for ServiceError {
    fn description(&self) -> &str {
        match *self {
            ServiceError::AccountAlreadyExists => "Account already exists",
            ServiceError::AuthorizeFailed => "Authorize failed",
        }
    }
}
