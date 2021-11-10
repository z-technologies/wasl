use crate::diesel::Connection;
use crate::repos::DbConnection;
use crate::result;

use crate::repos::admins_repo::AdminsRepo;
use crate::repos::groups_repo::GroupsRepo;
use crate::repos::users_repo::UsersRepo;

pub struct DbContext<'a> {
    db: &'a DbConnection,

    users: UsersRepo<'a>,
    admins: AdminsRepo<'a>,
    groups: GroupsRepo<'a>,
}

impl<'a> DbContext<'a> {
    pub fn new(db: &'a DbConnection) -> DbContext<'a> {
        DbContext::<'a> {
            db,

            users: UsersRepo::<'a> { db },
            admins: AdminsRepo::<'a> { db },
            groups: GroupsRepo::<'a> { db },
        }
    }

    pub fn connection(&self) -> &'a DbConnection {
        self.db
    }

    pub fn users(&self) -> &UsersRepo<'a> {
        &self.users
    }

    pub fn admins(&self) -> &AdminsRepo<'a> {
        &self.admins
    }

    pub fn groups(&self) -> &GroupsRepo<'a> {
        &self.groups
    }
}

pub fn create_connection() -> result::Result<DbConnection> {
    let url = std::env::var("DATABASE_URL")?;
    Ok(DbConnection::establish(&url)?)
}
