use data::result::DataError;

use argon2;
use derive_more::{Display, From};

use std::error;

pub type Result<T> = std::result::Result<T, UserError>;

#[derive(Debug, Display, From)]
pub enum InternalError {
    #[display(fmt = "data error: {}", _0)]
    DataError(DataError),

    #[display(fmt = "hashing error: {}", _0)]
    HashingError(argon2::Error),
}

#[derive(Debug, Display, From)]
pub enum UserError {
    #[display(fmt = "internal error: {}", _0)]
    InternalError(InternalError),
}

impl error::Error for InternalError {}
impl error::Error for UserError {}
