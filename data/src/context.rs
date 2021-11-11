use crate::repos::{DbConnectionManager, DbPool};
use crate::result;

use crate::repos::admins_repo::AdminsRepo;
use crate::repos::groups_repo::GroupsRepo;
use crate::repos::users_repo::UsersRepo;

#[derive(Clone)]
pub struct DbContext {
    users: UsersRepo,
    admins: AdminsRepo,
    groups: GroupsRepo,
}

impl DbContext {
    pub fn new(pool: DbPool) -> DbContext {
        DbContext {
            users: UsersRepo { pool: pool.clone() },
            admins: AdminsRepo { pool: pool.clone() },
            groups: GroupsRepo { pool }, // move
        }
    }

    pub fn users(&self) -> &UsersRepo {
        &self.users
    }

    pub fn admins(&self) -> &AdminsRepo {
        &self.admins
    }

    pub fn groups(&self) -> &GroupsRepo {
        &self.groups
    }
}

pub fn create_connection_pool(
    url: &str,
    max_connections: u32,
) -> result::Result<DbPool> {
    let manager = DbConnectionManager::new(url);

    match DbPool::builder().max_size(max_connections).build(manager) {
        Ok(pool) => Ok(pool),
        Err(err) => {
            Err(result::DataError::ConnectionPoolError(format!("{}", err)))
        }
    }
}
