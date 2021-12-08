use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum DataError {
    #[display(fmt = "database error: {:?}", _0)]
    DatabaseError(diesel::result::Error),

    #[display(fmt = "connection error: {:?}", _0)]
    ConnectionError(diesel::result::ConnectionError),

    #[display(fmt = "connection pool: {:?}", _0)]
    ConnectionPoolError(diesel::r2d2::PoolError),

    #[display(fmt = "not found")]
    NotFound,
}

pub type Result<T> = std::result::Result<T, DataError>;

pub fn adapt<T, E>(res: std::result::Result<T, E>) -> Result<T>
where
    DataError: From<E>,
{
    match res {
        Ok(res) => Ok(res),
        Err(err) => Err(DataError::from(err)),
    }
}
