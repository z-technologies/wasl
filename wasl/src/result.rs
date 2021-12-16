use argon2;
use base64;
use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, UserError>;

#[derive(Debug, Display, From)]
pub enum DataError {
    #[display(fmt = "database error: {:?}", _0)]
    DatabaseError(diesel::result::Error),

    #[display(fmt = "connection error: {:?}", _0)]
    ConnectionError(diesel::result::ConnectionError),

    #[display(fmt = "connection pool: {:?}", _0)]
    ConnectionPoolError(diesel::r2d2::PoolError),
}

#[derive(Debug, Display, From)]
pub enum EmailError {
    #[display(fmt = "general error: {}", _0)]
    GeneralError(lettre::error::Error),

    #[display(fmt = "transport error: {}", _0)]
    TransportError(lettre::transport::smtp::Error),

    #[display(fmt = "address error: {}", _0)]
    AddressError(lettre::address::AddressError),
}

#[derive(Debug, Display, From)]
pub enum InternalError {
    #[display(fmt = "io error: {}", _0)]
    IoError(std::io::Error),

    #[display(fmt = "environment error: {}", _0)]
    EnvironmentError(std::env::VarError),

    #[display(fmt = "data error: {}", _0)]
    DataError(DataError),

    #[display(fmt = "hashing error: {}", _0)]
    HashingError(argon2::Error),

    #[display(fmt = "utf8 error: {}", _0)]
    Utf8Error(std::str::Utf8Error),

    #[display(fmt = "base64 decode error: {}", _0)]
    Base64DecodeError(base64::DecodeError),

    #[display(fmt = "email error: {}", _0)]
    EmailError(EmailError),

    #[display(fmt = "tls error: {}", _0)]
    TlsError(native_tls::Error),
}

#[derive(Debug, Display, From)]
pub enum UserError {
    #[display(fmt = "internal error: {}", _0)]
    InternalError(InternalError),

    #[display(fmt = "not found")]
    NotFound,

    #[display(fmt = "permission denied")]
    PermissionDenied,

    #[display(fmt = "could not update account info")]
    CouldNotUpdateAccount,

    #[display(fmt = "invalid username or password")]
    InvalidUsernameOrPassword,

    #[display(fmt = "username is already in use")]
    UsernameAlreadyInUse,

    #[display(fmt = "email is already in use")]
    EmailAlreadyInUse,

    #[display(fmt = "invalid confirmation details")]
    InvalidConfirmationDetails,

    #[display(fmt = "time periods overlap")]
    TimePeriodsOverlap,

    #[display(fmt = "insufficient balance")]
    InsufficientBalance,
}

impl std::error::Error for InternalError {}
impl std::error::Error for UserError {}

impl From<std::io::Error> for UserError {
    fn from(err: std::io::Error) -> Self {
        UserError::InternalError(InternalError::IoError(err))
    }
}

impl From<std::env::VarError> for UserError {
    fn from(err: std::env::VarError) -> Self {
        UserError::InternalError(InternalError::EnvironmentError(err))
    }
}

impl From<argon2::Error> for UserError {
    fn from(err: argon2::Error) -> Self {
        UserError::InternalError(InternalError::HashingError(err))
    }
}

impl From<std::str::Utf8Error> for UserError {
    fn from(err: std::str::Utf8Error) -> Self {
        UserError::InternalError(InternalError::Utf8Error(err))
    }
}

impl From<base64::DecodeError> for UserError {
    fn from(err: base64::DecodeError) -> Self {
        UserError::InternalError(InternalError::Base64DecodeError(err))
    }
}

impl From<lettre::error::Error> for UserError {
    fn from(err: lettre::error::Error) -> Self {
        UserError::InternalError(InternalError::EmailError(
            EmailError::GeneralError(err),
        ))
    }
}

impl From<lettre::transport::smtp::Error> for UserError {
    fn from(err: lettre::transport::smtp::Error) -> Self {
        UserError::InternalError(InternalError::EmailError(
            EmailError::TransportError(err),
        ))
    }
}

impl From<lettre::address::AddressError> for UserError {
    fn from(err: lettre::address::AddressError) -> Self {
        UserError::InternalError(InternalError::EmailError(
            EmailError::AddressError(err),
        ))
    }
}

impl From<native_tls::Error> for UserError {
    fn from(err: native_tls::Error) -> Self {
        UserError::InternalError(InternalError::TlsError(err))
    }
}

impl From<diesel::result::Error> for UserError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => return UserError::NotFound,
            _ => UserError::InternalError(DataError::from(err).into()),
        }
    }
}

impl From<diesel::result::ConnectionError> for UserError {
    fn from(err: diesel::result::ConnectionError) -> Self {
        UserError::InternalError(DataError::from(err).into())
    }
}

impl From<diesel::r2d2::PoolError> for UserError {
    fn from(err: diesel::r2d2::PoolError) -> Self {
        UserError::InternalError(DataError::from(err).into())
    }
}
