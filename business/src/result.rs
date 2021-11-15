use data::result::DataError;

use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum InternalError {
    #[display(fmt = "data error: {}", _0)]
    DataError(DataError),
}

#[derive(Debug, Display, From)]
pub enum UserError {
    #[display(fmt = "internal error: {}", _0)]
    InternalError(InternalError),
}
