use data::result::DataError;

pub enum InternalError {
    DataError(DataError),
}

pub enum UserError {
    InternalError(InternalError),
}
