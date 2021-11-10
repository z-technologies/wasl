pub mod admins_repo;
pub mod users_repo;

use crate::models::KeyType;
use crate::result;
use diesel::PgConnection;

pub type DbConnection = PgConnection;

pub trait RepoTypes {
    type Model;
    type InsertModel;
}

pub trait Repo: RepoTypes {
    fn get_all(&self) -> result::Result<Vec<Self::Model>>;
    fn get(&self, id: KeyType) -> result::Result<Self::Model>;

    fn insert(&self, item: &Self::InsertModel) -> result::Result<Self::Model>;
    fn update<'a>(&self, item: &'a Self::Model) -> result::Result<&'a Self::Model>;

    fn delete(&self, item: &Self::Model) -> result::Result<()>;
}
