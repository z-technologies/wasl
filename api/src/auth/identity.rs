use crate::auth::token::Claims;
use crate::result::ApiError;

use actix_web::FromRequest;
use core::future::ready;
use core::future::Ready;

pub struct Identity(Claims);

impl FromRequest for Identity {
    type Error = ApiError;
    type Future = Ready<std::result::Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let result = match req.extensions().get::<Claims>().cloned() {
            Some(claims) => Ok(Identity(claims)),
            None => Err(ApiError::PermissionDenied),
        };

        ready(result)
    }
}

impl std::ops::Deref for Identity {
    type Target = Claims;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
