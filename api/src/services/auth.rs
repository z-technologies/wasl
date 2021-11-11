use crate::result::{ApiError, ApiResult};
use crate::security::password::password_matches;

use data::context::DbContext;
use data::models::user::User;

pub struct AuthSerivce<'ctx> {
    pub ctx: &'ctx DbContext,
}

impl<'ctx> AuthSerivce<'ctx> {
    pub fn signin<'a>(
        &self,
        username: &'a str,
        password: &'a str,
    ) -> ApiResult<User> {
        let user = self.ctx.users().get_by_username(username)?;

        if let Some(user) = user {
            Ok(Self::_signin_impl(user, password)?)
        } else {
            Err(ApiError::InvalidUsernameOrPassword)
        }
    }

    fn _signin_impl<'a>(user: User, password: &'a str) -> ApiResult<User> {
        if let Some(password_hash) = &user.password_hash {
            if let Some(password_salt) = &user.password_salt {
                if password_matches(password, &password_hash, &password_salt) {
                    return Ok(user);
                } else {
                    return Err(ApiError::InvalidUsernameOrPassword);
                }
            }
        }

        Err(ApiError::PasswordNotSet)
    }
}
