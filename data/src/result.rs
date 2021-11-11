use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum DataError {
    #[display(fmt = "Database Error: {:?}", _0)]
    DatabaseError(diesel::result::Error),

    #[display(fmt = "Connection Error: {:?}", _0)]
    ConnectionError(diesel::result::ConnectionError),

    #[display(fmt = "Connection Pool: {:?}", _0)]
    #[from(ignore)]
    ConnectionPoolError(String),

    #[display(fmt = "Environment Error: {:?}", _0)]
    EnvironmentError(std::env::VarError),
}

pub type Result<T> = std::result::Result<T, DataError>;
