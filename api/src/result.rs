use business::result::{InternalError, UserError};

use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{header, StatusCode};
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};
use validator::ValidationErrors;

#[derive(Debug, Display, From)]
pub enum ApiError {
    #[display(fmt = "internal error")]
    InternalError(InternalError),

    #[display(fmt = "user error: {}", _0)]
    UserError(UserError),

    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalSechulingError,

    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalEnvironmentError(std::env::VarError),

    #[display(fmt = "Validation error on field: {:?}", _0)]
    ValidationError(ValidationErrors),
}

pub type ApiResult<T> = std::result::Result<T, ApiError>;

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl<E: std::fmt::Debug> From<actix_web::error::BlockingError<E>> for ApiError {
    fn from(_: actix_web::error::BlockingError<E>) -> Self {
        ApiError::InternalSechulingError
    }
}
