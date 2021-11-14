use crate::repos::{DbConnectionManager, DbPool, GroupsRepo, UsersRepo};
use crate::result::{DataError, DataResult};

#[derive(Clone)]
pub struct DbContext {
    users: UsersRepo,
    groups: GroupsRepo,
}

impl DbContext {
    pub fn new(pool: DbPool) -> DbContext {
        DbContext {
            users: UsersRepo { pool: pool.clone() },
            groups: GroupsRepo { pool },
        }
    }

    pub fn users(&self) -> &UsersRepo {
        &self.users
    }

    pub fn groups(&self) -> &GroupsRepo {
        &self.groups
    }
}

pub fn create_connection_pool(
    url: &str,
    max_connections: u32,
) -> DataResult<DbPool> {
    let manager = DbConnectionManager::new(url);

    match DbPool::builder().max_size(max_connections).build(manager) {
        Ok(pool) => Ok(pool),
        Err(err) => Err(DataError::ConnectionPoolError(format!("{}", err))),
    }
}
