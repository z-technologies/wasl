use crate::models::{Group, NewUser, User, UserGroup};
use crate::repos::{DbPool, DbPooledConnection, Repo, RepoTypes};
use crate::result::{DataError, DataResult};

use data_derive::Repository;
use diesel::dsl::any;
use diesel::prelude::*;

#[derive(Clone, Repository)]
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
    ) -> DataResult<Option<User>> {
        use crate::schema::users::dsl::*;

        Ok(users
            .filter(username.eq(uname))
            .first::<User>(&self.get_connection()?)
            .optional()?)
    }

    pub fn get_by_email<'a>(&self, em: &'a str) -> DataResult<Option<User>> {
        use crate::schema::users::dsl::*;

        Ok(users
            .filter(email.eq(em))
            .first::<User>(&self.get_connection()?)
            .optional()?)
    }

    pub fn duplicate_username<'a>(&self, uname: &'a str) -> DataResult<bool> {
        use crate::schema::users::dsl::*;
        use diesel::dsl::*;

        Ok(select(exists(users.filter(username.eq(uname))))
            .get_result(&self.get_connection()?)?)
    }

    pub fn duplicate_email<'a>(&self, em: &'a str) -> DataResult<bool> {
        use crate::schema::users::dsl::*;
        use diesel::dsl::*;

        Ok(select(exists(users.filter(email.eq(em))))
            .get_result(&self.get_connection()?)?)
    }

    pub fn get_user_groups(&self, user: &User) -> DataResult<Vec<Group>> {
        use crate::schema::groups;
        use crate::schema::user_groups::dsl::*;

        let group_ids = UserGroup::belonging_to(user).select(group_id);

        Ok(groups::table
            .filter(groups::id.eq(any(group_ids)))
            .load::<Group>(&self.get_connection()?)?)
    }
}
