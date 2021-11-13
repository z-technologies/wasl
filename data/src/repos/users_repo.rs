use data_derive;

use crate::models::user::{NewUser, User};
use crate::repos::DbPooledConnection;
use crate::repos::{DbPool, Repo, RepoTypes};
use crate::result;

use diesel::prelude::*;

#[derive(Clone, data_derive::Repository)]
pub struct UsersRepo {
    pub pool: DbPool,
}

impl RepoTypes for UsersRepo {
    type Model = User;
    type InsertModel = NewUser;
}

impl UsersRepo {
    pub fn get_by_username<'a>(
        &self,
        uname: &'a str,
    ) -> result::Result<Option<User>> {
        use crate::schema::users::dsl::*;
        Ok(users
            .filter(username.eq(uname))
            .first::<User>(&self.get_connection()?)
            .optional()?)
    }

    pub fn get_by_email<'a>(
        &self,
        em: &'a str,
    ) -> result::Result<Option<User>> {
        use crate::schema::users::dsl::*;
        Ok(users
            .filter(email.eq(em))
            .first::<User>(&self.get_connection()?)
            .optional()?)
    }

    pub fn duplicate_username<'a>(
        &self,
        uname: &'a str,
    ) -> result::Result<bool> {
        use crate::schema::users::dsl::*;
        use diesel::dsl::*;

        Ok(select(exists(users.filter(username.eq(uname))))
            .get_result(&self.get_connection()?)?)
    }

    pub fn duplicate_email<'a>(&self, em: &'a str) -> result::Result<bool> {
        use crate::schema::users::dsl::*;
        use diesel::dsl::*;

        Ok(select(exists(users.filter(email.eq(em))))
            .get_result(&self.get_connection()?)?)
    }
}
