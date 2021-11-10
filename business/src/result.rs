use std::fmt;

#[derive(Debug)]
pub enum BusinessError {
    DataError(data::result::DataError),
    InvalidUsernameOrPassword,
}

pub type Result<T> = std::result::Result<T, BusinessError>;

impl fmt::Display for BusinessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            err => write!(f, "{:?}", err),
        }
    }
}

impl From<data::result::DataError> for BusinessError {
    fn from(err: data::result::DataError) -> DataError {
        BusinessError::DataError(err)
    }
}
