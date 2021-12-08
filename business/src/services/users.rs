use crate::result::Result;
use crate::result::UserError;
use data::context::DatabaseConnection;
use data::models::UserGroup;

use data::context::PostgresConnection;
use data::diesel::prelude::*;
use data::models::{Group, NewUser, User};

pub struct UsersService {
    conn: PostgresConnection,
}

impl UsersService {
    pub fn new(conn: PostgresConnection) -> UsersService {
        UsersService { conn }
    }

    pub fn get_by_username(&self, uname: &str) -> Result<User> {
        use data::schema::users::dsl::*;

        // TODO:
        // Properly handle errors

        Ok(users
            .filter(username.eq(uname))
            .first(&self.conn.get()?)
            .unwrap())
    }

    pub fn get_by_email(&self, em: &str) -> Result<User> {
        use data::schema::users::dsl::*;

        // TODO:
        // Properly handle errors

        Ok(users.filter(email.eq(em)).first(&self.conn.get()?).unwrap())
    }

    pub fn duplicate_username(&self, uname: &str) -> Result<bool> {
        use data::diesel::dsl::*;
        use data::schema::users::dsl::*;

        // TODO:
        // Properly handle errors

        Ok(select(exists(users.filter(username.eq(uname))))
            .get_result(&self.conn.get()?)
            .unwrap())
    }

    pub fn duplicate_email(&self, em: &str) -> Result<bool> {
        use data::diesel::dsl::*;
        use data::schema::users::dsl::*;

        // TODO:
        // Properly handle errors

        Ok(select(exists(users.filter(email.eq(em))))
            .get_result(&self.conn.get()?)
            .unwrap())
    }

    pub fn get_groups(&self, user: &User) -> Result<Vec<Group>> {
        use data::diesel::dsl::*;
        use data::schema::groups;
        use data::schema::user_groups::dsl::*;

        // TODO:
        // Properly handle errors

        let group_ids = UserGroup::belonging_to(user).select(group_id);

        Ok(groups::table
            .filter(groups::id.eq(any(group_ids)))
            .load(&self.conn.get()?)
            .unwrap())
    }

    pub fn create(&self, new_user: &NewUser) -> Result<User> {
        use data::schema::users::dsl::*;

        if self.duplicate_username(&new_user.username)? {
            return Err(UserError::UsernameAlreadyInUse);
        }

        if self.duplicate_email(&new_user.email)? {
            return Err(UserError::EmailAlreadyInUse);
        }

        // TODO:
        // Properly handle errors

        Ok(data::diesel::insert_into(users)
            .values(new_user)
            .get_result(&self.conn.get()?)
            .unwrap())
    }

    pub fn activate(&self, mut user: User) -> Result<User> {
        use data::schema::users::dsl::*;

        // TODO:
        // Properly handle errors

        Ok(data::diesel::update(users)
            .set(is_active.eq(true))
            .get_result(&self.conn.get()?)
            .unwrap())
    }

    pub fn delete(&self, user: User) -> Result<usize> {
        // TODO:
        // Properly handle errors

        Ok(data::diesel::delete(user)
            .execute(&self.conn.get()?)
            .unwrap())
    }
}
