use crate::models::{Group, User, UserGroup};
use crate::repos::{DbPool, DbPooledConnection, Repo};
use crate::result::Result;

use diesel::dsl::any;
use diesel::prelude::*;

#[derive(Clone)]
pub struct UsersRepo {
    pub pool: DbPool,
}

impl UsersRepo {
    pub fn get_by_username(&self, uname: &str) -> Result<Option<User>> {
        use crate::schema::users::dsl::*;

        Ok(users
            .filter(username.eq(uname))
            .first(&self.get_connection()?)
            .optional()?)
    }

    pub fn get_by_email(&self, em: &str) -> Result<Option<User>> {
        use crate::schema::users::dsl::*;

        Ok(users
            .filter(email.eq(em))
            .first(&self.get_connection()?)
            .optional()?)
    }

    pub fn duplicate_username(&self, uname: &str) -> Result<bool> {
        use crate::schema::users::dsl::*;
        use diesel::dsl::*;

        Ok(select(exists(users.filter(username.eq(uname))))
            .get_result(&self.get_connection()?)?)
    }

    pub fn duplicate_email(&self, em: &str) -> Result<bool> {
        use crate::schema::users::dsl::*;
        use diesel::dsl::*;

        Ok(select(exists(users.filter(email.eq(em))))
            .get_result(&self.get_connection()?)?)
    }

    pub fn get_user_groups(&self, user: &User) -> Result<Vec<Group>> {
        use crate::schema::groups;
        use crate::schema::user_groups::dsl::*;

        let group_ids = UserGroup::belonging_to(user).select(group_id);

        Ok(groups::table
            .filter(groups::id.eq(any(group_ids)))
            .load(&self.get_connection()?)?)
    }
}

impl Repo<User> for UsersRepo {
    fn get_connection(&self) -> Result<DbPooledConnection> {
        Ok(self.pool.get()?)
    }
}
