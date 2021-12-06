use crate::repos::*;
use crate::result::{DataError, Result};

#[derive(Clone)]
pub struct DbContext {
    users: UsersRepo,
    groups: GroupsRepo,
    confirmations: ConfirmationsRepo,
    services: ServicesRepo,
    products: ProductsRepo,
}

impl DbContext {
    pub fn new(pool: DbPool) -> DbContext {
        DbContext {
            users: UsersRepo { pool: pool.clone() },
            confirmations: ConfirmationsRepo { pool: pool.clone() },
            groups: GroupsRepo { pool: pool.clone() },
            services: ServicesRepo { pool: pool.clone() },
            products: ProductsRepo { pool: pool.clone() },
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

    pub fn services(&self) -> &ServicesRepo {
        &self.services
    }

    pub fn products(&self) -> &ProductsRepo {
        &self.products
    }
}

pub fn create_connection_pool(
    url: &str,
    max_connections: u32,
) -> Result<DbPool> {
    let manager = DbConnectionManager::new(url);

    Ok(DbPool::builder().max_size(max_connections).build(manager)?)
}
