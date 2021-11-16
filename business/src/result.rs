use data::result::DataError;

use argon2;
use base64;
use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, UserError>;

#[derive(Debug, Display, From)]
pub enum InternalError {
    #[display(fmt = "io error: {}", _0)]
    IoError(std::io::Error),

    #[display(fmt = "environment error: {}", _0)]
    EnvironmentError(std::env::VarError),

    #[display(fmt = "data error: {}", _0)]
    DataError(DataError),

    #[display(fmt = "hashing error: {}", _0)]
    HashingError(argon2::Error),

    #[display(fmt = "utf8 error: {}", _0)]
    Utf8Error(std::str::Utf8Error),

    #[display(fmt = "base64 decode error: {}", _0)]
    Base64DecodeError(base64::DecodeError),
}

#[derive(Debug, Display, From)]
pub enum UserError {
    #[display(fmt = "internal error: {}", _0)]
    InternalError(InternalError),

    #[display(fmt = "password not set")]
    PasswordNotSet,

    #[display(fmt = "invalid username or password")]
    InvalidUsernameOrPassword,

    #[display(fmt = "username is already in use")]
    UsernameAlreadyInUse,

    #[display(fmt = "email is already in use")]
    EmailAlreadyInUse,
}

impl std::error::Error for InternalError {}
impl std::error::Error for UserError {}

impl From<std::io::Error> for UserError {
    fn from(err: std::io::Error) -> Self {
        UserError::InternalError(InternalError::IoError(err))
    }
}

impl From<std::env::VarError> for UserError {
    fn from(err: std::env::VarError) -> Self {
        UserError::InternalError(InternalError::EnvironmentError(err))
    }
}

impl From<DataError> for UserError {
    fn from(err: DataError) -> Self {
        UserError::InternalError(InternalError::DataError(err))
    }
}

impl From<argon2::Error> for UserError {
    fn from(err: argon2::Error) -> Self {
        UserError::InternalError(InternalError::HashingError(err))
    }
}

impl From<std::str::Utf8Error> for UserError {
    fn from(err: std::str::Utf8Error) -> Self {
        UserError::InternalError(InternalError::Utf8Error(err))
    }
}

impl From<base64::DecodeError> for UserError {
    fn from(err: base64::DecodeError) -> Self {
        UserError::InternalError(InternalError::Base64DecodeError(err))
    }
}
