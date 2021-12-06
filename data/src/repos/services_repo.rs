use crate::models::Service;
use crate::repos::experimental::Repo;
use crate::repos::{DbPool, DbPooledConnection};
use crate::result::Result;

#[derive(Clone)]
pub struct ServicesRepo {
    pub pool: DbPool,
}

impl Repo<Service> for ServicesRepo {
    fn get_connection(&self) -> Result<DbPooledConnection> {
        Ok(self.pool.get()?)
    }
}
