use crate::repos::*;
use crate::result::{DataError, Result};

#[derive(Clone)]
pub struct DbContext {
    users: UsersRepo,
    groups: GroupsRepo,
    confirmations: ConfirmationsRepo,
}

impl DbContext {
    pub fn new(pool: DbPool) -> DbContext {
        DbContext {
            users: UsersRepo { pool: pool.clone() },
            confirmations: ConfirmationsRepo { pool: pool.clone() },
            groups: GroupsRepo { pool },
        }
    }

    pub fn users(&self) -> &UsersRepo {
        &self.users
    }

    pub fn confirmations(&self) -> &ConfirmationsRepo {
        &self.confirmations
    }

    pub fn groups(&self) -> &GroupsRepo {
        &self.groups
    }
}

pub fn create_connection_pool(
    url: &str,
    max_connections: u32,
) -> Result<DbPool> {
    let manager = DbConnectionManager::new(url);

    match DbPool::builder().max_size(max_connections).build(manager) {
        Ok(pool) => Ok(pool),
        Err(err) => Err(DataError::ConnectionPoolError(format!("{}", err))),
    }
}
