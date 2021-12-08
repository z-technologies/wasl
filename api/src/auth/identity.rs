use crate::auth::{groups::AuthGroup, token::Claims};
use crate::result::{ApiError, Result};

use wasl::data::models::User;
use wasl::services::UsersService;

use actix_web::{web, FromRequest};

use std::future::{ready, Ready};
use std::sync::Arc;

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

impl Identity {
    pub fn has(&self, group: AuthGroup) -> Result<bool> {
        if self.groups.iter().any(|g| g.name == group.to_string()) {
            Ok(true)
        } else {
            Err(ApiError::PermissionDenied)
        }
    }

    pub fn any(&self, groups: &[AuthGroup]) -> Result<()> {
        if groups
            .iter()
            .any(|g| self.groups.iter().any(|gg| gg.name == g.to_string()))
        {
            Ok(())
        } else {
            Err(ApiError::PermissionDenied)
        }
    }

    pub fn all(&self, groups: &[AuthGroup]) -> Result<()> {
        if groups
            .iter()
            .all(|g| self.groups.iter().any(|gg| gg.name == g.to_string()))
        {
            Ok(())
        } else {
            Err(ApiError::PermissionDenied)
        }
    }

    pub async fn user<'a>(self, users_svc: Arc<UsersService>) -> Result<User> {
        Ok(
            web::block(move || users_svc.get_by_username(&self.0.username))
                .await
                .unwrap(),
        )
    }
}

impl std::ops::Deref for Identity {
    type Target = Claims;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
