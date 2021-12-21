use wasl::result::{InternalError, UserError};

use actix_web::error::BlockingError;
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

    #[display(fmt = "jwt token error: {}", _0)]
    TokenError(jsonwebtoken::errors::Error),

    #[display(fmt = "permission denied")]
    PermissionDenied,

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
            ApiError::InternalError(..) | ApiError::SechulingError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }

            ApiError::TokenError(..) | ApiError::PermissionDenied => {
                StatusCode::UNAUTHORIZED
            }

            ApiError::ValidationError { .. } => StatusCode::BAD_REQUEST,

            ApiError::UserError(err) => match err {
                UserError::InternalError(..)
                | UserError::CouldNotUpdateAccount => {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
                UserError::NotFound | UserError::OutOfStock => {
                    StatusCode::NOT_FOUND
                }
                UserError::InvalidConfirmationDetails
                | UserError::InsufficientBalance => StatusCode::FORBIDDEN,
                UserError::PermissionDenied
                | UserError::InvalidUsernameOrPassword => {
                    StatusCode::UNAUTHORIZED
                }
                UserError::UsernameAlreadyInUse
                | UserError::EmailAlreadyInUse
                | UserError::TimePeriodsOverlap => StatusCode::CONFLICT,
            },
        }
    }
}

impl From<BlockingError<UserError>> for ApiError {
    fn from(err: BlockingError<UserError>) -> Self {
        match err {
            BlockingError::Error(err) => ApiError::UserError(err),
            BlockingError::Canceled => ApiError::SechulingError,
        }
    }
}
