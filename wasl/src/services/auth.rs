use crate::data::models::{Confirmation, Group, NewUser, User};
use crate::result::{Result, UserError};
use crate::security::password::is_match;
use crate::services::email::{make_mail_box, EmailService};
use crate::services::{ConfirmationsService, UsersService};

use std::sync::Arc;

pub struct AuthSerivce {
    users_svc: Arc<UsersService>,
    confirmations_svc: Arc<ConfirmationsService>,
    email_svc: Arc<EmailService>,
}

impl AuthSerivce {
    pub fn new(
        users_svc: Arc<UsersService>,
        confirmations_svc: Arc<ConfirmationsService>,
        email_svc: Arc<EmailService>,
    ) -> AuthSerivce {
        AuthSerivce {
            users_svc,
            confirmations_svc,
            email_svc,
        }
    }

    pub fn signin(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(User, Vec<Group>)> {
        let user = self.users_svc.get_by_username(username)?;

        if !user.is_active {
            Err(UserError::NotFound)
        } else if !is_match(password, &user.password_hash)? {
            Err(UserError::InvalidUsernameOrPassword)
        } else {
            let groups = self.users_svc.get_groups(&user)?;
            Ok((user, groups))
        }
    }

    pub fn signup(&self, new_user: &NewUser) -> Result<User> {
        let user = self.users_svc.create(new_user)?;

        match self.send_verification_email(&user) {
            Ok(..) => Ok(user),
            Err(err) => {
                self.users_svc.delete(user)?;
                Err(err)
            }
        }
    }

    fn send_verification_email(&self, user: &User) -> Result<()> {
        let conf = self
            .confirmations_svc
            .generate_for::<8, 64>(user, chrono::Duration::minutes(30))?;

        let ret = self.email_svc.send_noreply(
            "Account Confirmation",
            build_confirmation_email(user, &conf),
            make_mail_box(&user.username, &user.email)?,
        );

        if let Err(err) = ret {
            self.confirmations_svc.delete(conf)?;
            return Err(err);
        }

        Ok(())
    }

    pub fn activate_with_token(
        &self,
        username: &str,
        token: &str,
    ) -> Result<()> {
        let conf = self.confirmations_svc.get_by_token(token)?;
        let user = self.users_svc.get_by_username(username)?;

        Ok(self
            .confirmations_svc
            .confirm(conf, user, |c| c.token == token)?)
    }

    pub fn activate_with_otp(&self, username: &str, otp: &str) -> Result<()> {
        let conf = self.confirmations_svc.get_by_otp(otp)?;
        let user = self.users_svc.get_by_username(username)?;

        Ok(self
            .confirmations_svc
            .confirm(conf, user, |c| c.otp == otp)?)
    }
}

fn build_confirmation_email(user: &User, conf: &Confirmation) -> String {
    let html = format!(
        r#"
<p>
    Hello, <b>{username}</b>!
</p>

<p style="line-height: 2em">
    Your activation code is:
    <b style="color:#414141; background-color: #efefef; padding: 8px 16px">
        {otp}
    </b>
    <br>
    Activate directly
    <a href="http://localhost:8080/api/v1/auth/activate/{token}">
        from here
    </a>
    <br>
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
