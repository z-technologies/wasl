use crate::result::Result;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

pub use diesel::Connection;

pub trait DatabaseConnection {
    type Conn;

    fn get(&self) -> Result<Self::Conn>;
}

#[derive(Clone)]
pub struct PostgresConnection {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresConnection {
    pub fn new(url: &str) -> Result<PostgresConnection> {
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = Pool::builder().build(manager)?;

        Ok(PostgresConnection { pool })
    }
}

impl DatabaseConnection for PostgresConnection {
    type Conn = PooledConnection<ConnectionManager<PgConnection>>;

    fn get(&self) -> Result<Self::Conn> {
        Ok(self.pool.get()?)
    }
}
