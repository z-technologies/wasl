use crate::models::KeyType;
use crate::result::DataResult;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

pub type DbConnection = PgConnection;
pub type DbConnectionManager = ConnectionManager<DbConnection>;

pub type DbPooledConnection = PooledConnection<DbConnectionManager>;
pub type DbPool = Pool<DbConnectionManager>;

pub trait RepoTypes {
    type Model;
    type InsertModel;
}

pub trait Repo: RepoTypes {
    fn get_all(&self) -> DataResult<Vec<Self::Model>>;
    fn get(&self, id: KeyType) -> DataResult<Self::Model>;

    fn insert(&self, item: &Self::InsertModel) -> DataResult<Self::Model>;
    fn update<'a>(&self, item: &'a Self::Model) -> DataResult<&'a Self::Model>;

    fn delete(&self, item: &Self::Model) -> DataResult<()>;
    fn get_connection(&self) -> DataResult<DbPooledConnection>;
}
