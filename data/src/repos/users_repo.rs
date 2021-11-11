use data_derive;

use crate::models::user::{NewUser, User};
use crate::repos::DbPooledConnection;
use crate::repos::{DbPool, Repo, RepoTypes};
use crate::result;

use diesel::prelude::*;

#[derive(data_derive::Repository)]
pub struct UsersRepo<'db> {
    pub pool: &'db DbPool,
}

impl<'db> RepoTypes for UsersRepo<'db> {
    type Model = User;
    type InsertModel = NewUser<'db>;
}

impl<'db> UsersRepo<'db> {
    pub fn get_by_username<'a>(&self, uname: &'a str) -> result::Result<User> {
        use crate::schema::users::dsl::*;
        Ok(users
            .filter(username.eq(uname))
            .first::<User>(&self.get_connection()?)?)
    }

    pub fn get_by_email<'a>(&self, em: &'a str) -> result::Result<User> {
        use crate::schema::users::dsl::*;
        Ok(users
            .filter(email.eq(em))
            .first::<User>(&self.get_connection()?)?)
    }
}
