use std::fmt;

#[derive(Debug)]
pub enum DataError {
    DatabaseError(diesel::result::Error),
    ConnectionError(diesel::result::ConnectionError),
}

pub type Result<T> = std::result::Result<T, DataError>;

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::DatabaseError(err) => write!(f, "Database Error: {}", err),
            DataError::ConnectionError(err) => write!(f, "Connection Error: {}", err),
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
