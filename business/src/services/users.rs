use crate::result::Result;
use crate::result::UserError;

use data::connection::*;
use data::diesel::prelude::*;
use data::models::{Group, NewUser, User, UserGroup};

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
        // Properly handle NotFound error

        data::result::adapt(
            users
                .filter(username.eq(uname))
                .first(&self.conn.get()?)
                .optional(),
        )?
        .ok_or(UserError::NotFound)
    }

    pub fn get_by_email(&self, em: &str) -> Result<User> {
        use data::schema::users::dsl::*;

        // TODO:
        // Properly handle NotFound error

        data::result::adapt(
            users
                .filter(email.eq(em))
                .first(&self.conn.get()?)
                .optional(),
        )?
        .ok_or(UserError::NotFound)
    }

    pub fn username_exists(&self, uname: &str) -> Result<bool> {
        use data::diesel::dsl::*;
        use data::schema::users::dsl::*;

        Ok(data::result::adapt(
            select(exists(users.filter(username.eq(uname))))
                .get_result(&self.conn.get()?),
        )?)
    }

    pub fn email_exists(&self, em: &str) -> Result<bool> {
        use data::diesel::dsl::*;
        use data::schema::users::dsl::*;

        Ok(data::result::adapt(
            select(exists(users.filter(email.eq(em))))
                .get_result(&self.conn.get()?),
        )?)
    }

    pub fn get_groups(&self, user: &User) -> Result<Vec<Group>> {
        use data::diesel::dsl::*;
        use data::schema::groups;
        use data::schema::user_groups::dsl::*;

        let group_ids = UserGroup::belonging_to(user).select(group_id);

        Ok(data::result::adapt(
            groups::table
                .filter(groups::id.eq(any(group_ids)))
                .load(&self.conn.get()?),
        )?)
    }

    pub fn create(&self, new_user: &NewUser) -> Result<User> {
        use data::schema::users::dsl::*;

        if self.username_exists(&new_user.username)? {
            return Err(UserError::UsernameAlreadyInUse);
        }

        if self.email_exists(&new_user.email)? {
            return Err(UserError::EmailAlreadyInUse);
        }

        Ok(data::result::adapt(
            data::diesel::insert_into(users)
                .values(new_user)
                .get_result(&self.conn.get()?),
        )?)
    }

    pub fn activate(&self, user: User) -> Result<User> {
        use data::schema::users::dsl::*;

        Ok(data::result::adapt(
            data::diesel::update(&user)
                .set(is_active.eq(true))
                .get_result(&self.conn.get()?),
        )?)
    }

    pub fn delete(&self, user: User) -> Result<usize> {
        Ok(data::result::adapt(
            data::diesel::delete(&user).execute(&self.conn.get()?),
        )?)
    }
}
