use crate::result::{Result, UserError};
use crate::security::password::{is_match, make_hash};

use data::context::DbContext;
use data::models::{Group, NewUser, User};
use data::repos::Repo;

pub struct AuthSerivce {
    ctx: DbContext,
}

impl AuthSerivce {
    pub fn new(ctx: DbContext) -> AuthSerivce {
        AuthSerivce { ctx }
    }

    pub fn signin<'a>(
        &self,
        username: &'a str,
        password: &'a str,
    ) -> Result<(User, Vec<Group>)> {
        let user = self.ctx.users().get_by_username(username)?;

        if let Some(user) = user {
            if let Some(hash) = &user.password_hash {
                let groups = self.ctx.users().get_user_groups(&user)?;

                if is_match(password, &hash)? {
                    Ok((user, groups))
                } else {
                    Err(UserError::InvalidUsernameOrPassword)
                }
            } else {
                Err(UserError::PasswordNotSet)
            }
        } else {
            Err(UserError::InvalidUsernameOrPassword)
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

    pub fn set_initial_password<'a>(
        &self,
        username: &'a str,
        password: &'a str,
        _token: &'a str,
    ) -> Result<()> {
        // TODO:
        // validate token

        let user = self.ctx.users().get_by_username(username)?;

        if let Some(mut user) = user {
            if user.is_active {
                Err(UserError::CouldNotUpdateAccount)
            } else {
                user.password_hash = Some(make_hash(password)?);
                self.ctx.users().update(&user)?;
                Ok(())
            }
        } else {
            Err(UserError::NotFound)
        }
    }
}
