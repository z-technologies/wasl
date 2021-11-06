use diesel;

pub enum DataError {
    InternalError(diesel::result::Error),
    NotFound,
}

pub type Result<T> = std::result::Result<T, DataError>;
