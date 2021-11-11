use crate::models::KeyType;
use crate::schema::{admin_groups, admins};

use crate::models::group::Group;

use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Admin)]
#[belongs_to(Group)]
pub struct AdminGroup {
    pub id: KeyType,
    pub admin_id: KeyType,
    pub group_id: KeyType,
}

#[derive(Clone, Debug, Identifiable, Queryable, AsChangeset)]
pub struct Admin {
    pub id: KeyType,

    pub username: String,
    pub password_hash: String,
    pub password_salt: String,

    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "admins"]
pub struct NewAdmin {
    pub username: String,
    pub password_hash: String,
    pub password_salt: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
