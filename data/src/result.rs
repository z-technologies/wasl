use std::fmt;

#[derive(Debug)]
pub enum DataError {
    DatabaseError(diesel::result::Error),
    ConnectionError(diesel::result::ConnectionError),
    EnvironmentError(std::env::VarError),
}

pub type Result<T> = std::result::Result<T, DataError>;

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            err => write!(f, "{:?}", err),
        }
    }
}

impl From<diesel::result::Error> for DataError {
    fn from(err: diesel::result::Error) -> DataError {
        DataError::DatabaseError(err)
    }
}

impl From<diesel::result::ConnectionError> for DataError {
    fn from(err: diesel::result::ConnectionError) -> DataError {
        DataError::ConnectionError(err)
    }
}

impl From<std::env::VarError> for DataError {
    fn from(err: std::env::VarError) -> DataError {
        DataError::EnvironmentError(err)
    }
}
