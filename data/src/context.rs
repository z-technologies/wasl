use crate::diesel::Connection;
use crate::repos::users_repo::UsersRepo;
use crate::repos::DbConnection;
use crate::result;

pub struct DbContext<'a> {
    db: &'a DbConnection,
    users: UsersRepo<'a>,
}

impl<'a> DbContext<'a> {
    pub fn new(db: &'a DbConnection) -> DbContext<'a> {
        DbContext::<'a> {
            db,
            users: UsersRepo::<'a>::new(db),
        }
    }

    pub fn connection(&self) -> &'a DbConnection {
        self.db
    }

    pub fn users(&self) -> &UsersRepo<'a> {
        &self.users
    }
}

pub fn create_connection(url: &str) -> result::Result<DbConnection> {
    Ok(DbConnection::establish(&url)?)
}
