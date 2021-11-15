use data::result::DataError;

use argon2;
use base64;
use derive_more::{Display, From};

use std::error;
use std::str::Utf8Error;

pub type Result<T> = std::result::Result<T, UserError>;

#[derive(Debug, Display, From)]
pub enum InternalError {
    #[display(fmt = "data error: {}", _0)]
    DataError(DataError),

    #[display(fmt = "hashing error: {}", _0)]
    HashingError(argon2::Error),

    #[display(fmt = "utf8 error: {}", _0)]
    Utf8Error(Utf8Error),

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
}

impl error::Error for InternalError {}
impl error::Error for UserError {}

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

impl From<Utf8Error> for UserError {
    fn from(err: Utf8Error) -> Self {
        UserError::InternalError(InternalError::Utf8Error(err))
    }
}

impl From<base64::DecodeError> for UserError {
    fn from(err: base64::DecodeError) -> Self {
        UserError::InternalError(InternalError::Base64DecodeError(err))
    }
}
