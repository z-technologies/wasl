use data::context::DbContext;
use data::models::user::User;

use crate::result;
use crate::security::password::password_matches;

struct AuthSerivce<'ctx> {
    ctx: &'ctx DbContext<'ctx>,
}

impl<'ctx> AuthSerivce<'ctx> {
    pub fn login_with_username_and_password<'a>(
        &self,
        username: &'a str,
        password: &'a str,
    ) -> result::Result<User> {
        let user = self.ctx.users().get_by_username(username)?;
        Self::login_with_password(user, password)
    }

    fn login_with_password<'a>(user: User, password: &'a str) -> result::Result<User> {
        if let Some(password_hash) = &user.password_hash {
            if let Some(password_salt) = &user.password_salt {
                if password_matches(password, &password_hash, &password_salt) {
                    return Ok(user);
                } else {
                    return Err(result::BusinessError::InvalidPassword);
                }
            }
        }

        Err(result::BusinessError::NoPasswordIsSet)
    }
}
