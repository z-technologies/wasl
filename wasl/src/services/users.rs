use crate::data::connection::*;
use crate::data::models::{Group, KeyType, NewUser, User, UserGroup};
use crate::result::Result;

use diesel::prelude::*;

pub struct UsersService {
    conn: PostgresConnection,
}

impl UsersService {
    pub fn new(conn: PostgresConnection) -> UsersService {
        UsersService { conn }
    }

    pub fn get_by_id(&self, uid: KeyType) -> Result<User> {
        use crate::data::schema::users::dsl::*;

        Ok(users.find(uid).first(&self.conn.get()?)?)
    }

    pub fn get_by_username(&self, uname: &str) -> Result<User> {
        use crate::data::schema::users::dsl::*;

        Ok(users.filter(username.eq(uname)).first(&self.conn.get()?)?)
    }

    pub fn get_by_email(&self, em: &str) -> Result<User> {
        use crate::data::schema::users::dsl::*;

        Ok(users.filter(email.eq(em)).first(&self.conn.get()?)?)
    }

    pub fn username_exists(&self, uname: &str) -> Result<bool> {
        use crate::data::schema::users::dsl::*;
        use diesel::dsl::*;

        Ok(select(exists(users.filter(username.eq(uname))))
            .get_result(&self.conn.get()?)?)
    }

    pub fn email_exists(&self, em: &str) -> Result<bool> {
        use crate::data::schema::users::dsl::*;
        use diesel::dsl::*;

        Ok(select(exists(users.filter(email.eq(em))))
            .get_result(&self.conn.get()?)?)
    }

    pub fn get_groups(&self, user: &User) -> Result<Vec<Group>> {
        use crate::data::schema::groups;
        use crate::data::schema::user_groups::dsl::*;
        use diesel::dsl::*;

        let group_ids = UserGroup::belonging_to(user).select(group_id);

        Ok(groups::table
            .filter(groups::id.eq(any(group_ids)))
            .load(&self.conn.get()?)?)
    }

    pub fn create(&self, item: &NewUser) -> Result<User> {
        use crate::data::schema::users::dsl::*;

        Ok(diesel::insert_into(users)
            .values(item)
            .get_result(&self.conn.get()?)?)
    }

    pub fn activate(&self, user: User) -> Result<User> {
        use crate::data::schema::users::dsl::*;

        Ok(diesel::update(&user)
            .set(is_active.eq(true))
            .get_result(&self.conn.get()?)?)
    }

    pub fn delete(&self, user: User) -> Result<usize> {
        Ok(diesel::delete(&user).execute(&self.conn.get()?)?)
    }
}
