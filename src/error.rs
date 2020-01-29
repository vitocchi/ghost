use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    AccountAlreadyExists,
    AuthorizeFailed,
    AccountNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::AccountAlreadyExists => f.write_str("AccountAlreadyExists"),
            Error::AuthorizeFailed => f.write_str("AuthorizeFailed"),
            Error::AccountNotFound => f.write_str("AccountNotFound"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AccountAlreadyExists => "Account already exists",
            Error::AuthorizeFailed => "Authorize failed",
            Error::AccountNotFound => "Account not found",
        }
    }
}
