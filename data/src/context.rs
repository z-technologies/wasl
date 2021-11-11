use crate::repos::{DbConnectionManager, DbPool};
use crate::result;

use crate::repos::admins_repo::AdminsRepo;
use crate::repos::groups_repo::GroupsRepo;
use crate::repos::users_repo::UsersRepo;

pub struct DbContext<'db> {
    pub pool: &'db DbPool,

    users: UsersRepo<'db>,
    admins: AdminsRepo<'db>,
    groups: GroupsRepo<'db>,
}

impl<'a> DbContext<'a> {
    pub fn new(pool: &'a DbPool) -> DbContext<'a> {
        DbContext::<'a> {
            pool,
            users: UsersRepo::<'a> { pool },
            admins: AdminsRepo::<'a> { pool },
            groups: GroupsRepo::<'a> { pool },
        }
    }

    pub fn users(&self) -> &UsersRepo<'a> {
        &self.users
    }

    pub fn admins(&self) -> &AdminsRepo<'a> {
        &self.admins
    }

    pub fn groups(&self) -> &GroupsRepo<'a> {
        &self.groups
    }
}

pub fn create_connection_pool(url: &str) -> result::Result<DbPool> {
    let manager = DbConnectionManager::new(url);

    match DbPool::builder().build(manager) {
        Ok(pool) => Ok(pool),
        Err(err) => Err(result::DataError::ConnectionPoolError(format!("{}", err))),
    }
}
