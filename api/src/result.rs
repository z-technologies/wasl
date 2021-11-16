use business::result::{InternalError, UserError};

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};
use validator::ValidationErrors;

#[derive(Debug, Display, From)]
pub enum ApiError {
    #[display(fmt = "internal error: {}", _0)]
    InternalError(InternalError),

    #[display(fmt = "user error: {}", _0)]
    UserError(UserError),

    #[display(fmt = "scheduling error")]
    SechulingError,

    #[display(fmt = "validation error on field(s): {}", _0)]
    ValidationError(ValidationErrors),
}

pub type Result<T> = std::result::Result<T, ApiError>;

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        if self.status_code() == StatusCode::INTERNAL_SERVER_ERROR {
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish()
        } else {
            HttpResponse::build(self.status_code()).body(self.to_string())
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InternalError(_) | ApiError::SechulingError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }

            ApiError::ValidationError { .. } => StatusCode::BAD_REQUEST,

            ApiError::UserError(err) => match err {
                UserError::InternalError(..) => {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
                UserError::PasswordNotSet => StatusCode::FORBIDDEN,
                UserError::InvalidUsernameOrPassword => {
                    StatusCode::UNAUTHORIZED
                }
                UserError::UsernameAlreadyInUse
                | UserError::EmailAlreadyInUse => StatusCode::CONFLICT,
            },
        }
    }
}

impl<E: std::fmt::Debug> From<actix_web::error::BlockingError<E>> for ApiError {
    fn from(_: actix_web::error::BlockingError<E>) -> Self {
        ApiError::SechulingError
    }
}
