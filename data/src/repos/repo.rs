use crate::models::KeyType;
use crate::result::Result;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

pub type DbConnection = PgConnection;
pub type DbConnectionManager = ConnectionManager<DbConnection>;

pub type DbPooledConnection = PooledConnection<DbConnectionManager>;
pub type DbPool = Pool<DbConnectionManager>;

pub trait Repo {
    type Model;
    type InsertModel;

    fn get_all(&self) -> Result<Vec<Self::Model>>;
    fn get(&self, id: KeyType) -> Result<Self::Model>;

    fn insert(&self, item: &Self::InsertModel) -> Result<Self::Model>;
    fn update<'a>(&self, item: &'a Self::Model) -> Result<&'a Self::Model>;

    fn delete(&self, item: &Self::Model) -> Result<()>;
    fn get_connection(&self) -> Result<DbPooledConnection>;
}
