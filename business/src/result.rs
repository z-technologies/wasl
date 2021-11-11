use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum BusinessError {
    #[display(fmt = "Data Error: {}", _0)]
    DataError(data::result::DataError),

    #[display(fmt = "invalid password")]
    InvalidPassword,

    #[display(fmt = "no password is set for the user")]
    NoPasswordIsSet,
}

pub type Result<T> = std::result::Result<T, BusinessError>;
