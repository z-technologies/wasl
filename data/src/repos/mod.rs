pub mod users_repo;

use crate::models::KeyType;
use crate::result;
use diesel::PgConnection;

pub type DbConnection = PgConnection;

pub trait Repo<T, I> {
    fn get_all(&self) -> result::Result<Vec<T>>;
    fn get(&self, id: KeyType) -> result::Result<T>;

    fn insert(&self, item: &I) -> result::Result<T>;
    fn update<'a>(&self, item: &'a T) -> result::Result<&'a T>;

    fn delete(&self, item: &T) -> result::Result<()>;
}
