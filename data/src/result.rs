use std::fmt;

#[derive(Debug)]
pub enum DataError {
    InternalError(diesel::result::Error),
    ConnectionError(diesel::result::ConnectionError),
    NotFound,
}

pub type Result<T> = std::result::Result<T, DataError>;

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::InternalError(err) => write!(f, "{}", err),
            DataError::ConnectionError(err) => write!(f, "{}", err),
            DataError::NotFound => write!(f, "no such item"),
        }
    }
}

impl From<diesel::result::Error> for DataError {
    fn from(err: diesel::result::Error) -> DataError {
        DataError::InternalError(err)
    }
}

impl From<diesel::result::ConnectionError> for DataError {
    fn from(err: diesel::result::ConnectionError) -> DataError {
        DataError::ConnectionError(err)
    }
}
