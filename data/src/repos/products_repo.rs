use crate::models::Product;
use crate::repos::experimental::Repo;
use crate::repos::{DbPool, DbPooledConnection};
use crate::result::Result;

#[derive(Clone)]
pub struct ProductsRepo {
    pub pool: DbPool,
}

impl Repo<Product> for ProductsRepo {
    fn get_connection(&self) -> Result<DbPooledConnection> {
        Ok(self.pool.get()?)
    }
}
