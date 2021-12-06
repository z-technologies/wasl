use crate::models::Group;
use crate::repos::experimental::Repo;
use crate::repos::{DbPool, DbPooledConnection};
use crate::result::Result;

#[derive(Clone)]
pub struct GroupsRepo {
    pub pool: DbPool,
}

impl Repo<Group> for GroupsRepo {
    fn get_connection(&self) -> Result<DbPooledConnection> {
        Ok(self.pool.get()?)
    }
}
