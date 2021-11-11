use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{header, StatusCode};
use actix_web::{HttpResponse, ResponseError};

use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum ApiError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalDataError(data::result::DataError),

    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalBusinessError(business::result::BusinessError),

    #[display(fmt = "Invalid username or password")]
    InvalidUsernameOrPassword,

    #[display(fmt = "Password is not set")]
    PasswordNotSet,

    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
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
            ApiError::InternalDataError(..)
            | ApiError::InternalBusinessError(..) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }

            ApiError::InvalidUsernameOrPassword => StatusCode::UNAUTHORIZED,
            ApiError::PasswordNotSet => StatusCode::FORBIDDEN,
            ApiError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
