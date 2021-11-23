use crate::result::{Result, UserError};
use crate::security::password::{is_match, make_hash};
use crate::security::random::generate_alphanum_string;
use crate::services::email::{make_mail_box, EmailService};

use data::context::DbContext;
use data::models::{Confirmation, Group, NewConfirmation, NewUser, User};
use data::repos::Repo;

use std::sync::Arc;

pub struct AuthSerivce {
    ctx: DbContext,
    email_svc: Arc<EmailService>,
}

impl AuthSerivce {
    pub fn new(ctx: DbContext, email_svc: Arc<EmailService>) -> AuthSerivce {
        AuthSerivce { ctx, email_svc }
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
        self.send_verification_email(&user)?;

        Ok(user)
    }

    pub fn set_initial_password<'a>(
        &self,
        username: &'a str,
        password: &'a str,
        token: &'a str,
    ) -> Result<()> {
        let user = self.ctx.users().get_by_username(username)?;

        if let Some(mut user) = user {
            let conf = self.validate_token(&user, token)?;

            if user.is_active {
                Err(UserError::CouldNotUpdateAccount)
            } else {
                user.password_hash = Some(make_hash(password)?);
                self.ctx.confirmations().delete(&conf)?;
                self.ctx.users().update(&user)?;
                Ok(())
            }
        } else {
            Err(UserError::NotFound)
        }
    }

    pub fn send_verification_email(&self, user: &User) -> Result<()> {
        let conf =
            self.create_confirmation(user, chrono::Duration::minutes(30))?;

        let ret = self.email_svc.send_noreply(
            "Account Confirmation",
            build_confirmation_email(user, &conf),
            make_mail_box(&user.username, &user.email)?,
        );

        if let Err(err) = ret {
            self.ctx.confirmations().delete(&conf)?;
            return Err(err);
        }

        Ok(())
    }

    fn create_confirmation(
        &self,
        user: &User,
        valid_for: chrono::Duration,
    ) -> Result<Confirmation> {
        let conf = NewConfirmation {
            user_id: user.id,
            otp: generate_alphanum_string::<8>(),
            token: generate_alphanum_string::<64>(),
            issued_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + valid_for,
        };

        Ok(self.ctx.confirmations().insert(&conf)?)
    }

    fn validate_token(&self, user: &User, token: &str) -> Result<Confirmation> {
        let conf = self.ctx.confirmations().get_by_token(token)?;

        if let Some(conf) = conf {
            if conf.user_id == user.id && conf.token == token {
                Ok(conf)
            } else {
                Err(UserError::InvalidConfirmationDetails)
            }
        } else {
            Err(UserError::InvalidConfirmationDetails)
        }
    }
}

fn build_confirmation_email(user: &User, conf: &Confirmation) -> String {
    let html = format!(
        r#"
<p>
    Hello, <b>{username}</b>!
</p>

<p style="line-height: 2em">
	Your activation code is: <b style="color:#414141; background-color: #efefef; padding: 8px 16px">{otp}</b><br>
	Activate directly <a href="http://localhost:8080/api/v1/auth/activate/{token}">from here</a> <br>
</p>
  
<footer>This email expires on <b>{expires}.</footer>
        "#,
        username = user.username,
        otp = conf.otp,
        token = conf.token,
        expires = conf.expires_at
    );

    html
}
