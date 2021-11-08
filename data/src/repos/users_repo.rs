use crate::models::user::{NewUser, User};
use crate::models::KeyType;
use crate::repos::{DbConnection, Repo};
use crate::result;
use crate::result::DataError;

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct UsersRepo<'a> {
    db: &'a DbConnection,
}

impl<'db> Repo<User, NewUser<'_>> for UsersRepo<'db> {
    fn get_all(&self) -> result::Result<Vec<User>> {
        use crate::schema::users::dsl::*;
        Ok(users.load::<User>(self.db)?)
    }

    fn get(&self, key: KeyType) -> result::Result<User> {
        use crate::schema::users::dsl::*;
        Ok(users.filter(id.eq(key)).get_result(self.db)?)
    }

    fn insert<'a>(&self, item: &'a NewUser) -> result::Result<User> {
        use crate::schema::users::dsl::*;
        Ok(diesel::insert_into(users)
            .values(item)
            .get_result::<User>(self.db)?)
    }

    fn update<'a>(&self, item: &'a User) -> result::Result<&'a User> {
        use crate::schema::users::dsl::*;
        diesel::update(users).set(item).execute(self.db)?;
        Ok(item)
    }

    fn delete(&self, item: &User) -> result::Result<()> {
        use crate::schema::users::dsl::*;
        match diesel::delete(users.filter(id.eq(item.id))).execute(self.db) {
            Ok(_) => Ok(()),
            Err(err) => Err(DataError::from(err)),
        }
    }
}

impl<'a> UsersRepo<'a> {
    pub fn new(db: &'a DbConnection) -> UsersRepo<'a> {
        UsersRepo::<'a> { db }
    }
}
