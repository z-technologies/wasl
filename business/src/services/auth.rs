use crate::result::{Result, UserError};
use crate::security::password::is_match;

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
    ) -> Result<User> {
        let user = self.ctx.users().get_by_username(username)?;

        match user {
            Some(user) => Ok(signin_impl(user, password)?),
            None => Err(UserError::InvalidUsernameOrPassword),
        }
    }
}

fn signin_impl<'a>(user: User, password: &'a str) -> Result<User> {
    match &user.password_hash {
        Some(hash) => {
            if is_match(password, &hash)? {
                Ok(user)
            } else {
                Err(UserError::InvalidUsernameOrPassword)
            }
        }
        None => Err(UserError::PasswordNotSet),
    }
}
