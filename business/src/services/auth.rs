use crate::result::{Result, UserError};
use crate::security::password::is_match;

use data::context::DbContext;
use data::models::{NewUser, User};
use data::repos::Repo;

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
            Some(user) => Ok(do_signin(user, password)?),
            None => Err(UserError::InvalidUsernameOrPassword),
        }
    }

    pub fn signup<'a>(&self, new_user: &'a NewUser) -> Result<User> {
        if self.ctx.users().duplicate_username(&new_user.username)? {
            return Err(UserError::UsernameAlreadyInUse);
        }

        if self.ctx.users().duplicate_email(&new_user.email)? {
            return Err(UserError::EmailAlreadyInUse);
        }

        let user = self.ctx.users().insert(&new_user)?;

        // TODO:
        // handle email verification

        Ok(user)
    }
}

fn do_signin<'a>(user: User, password: &'a str) -> Result<User> {
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
