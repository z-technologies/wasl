use crate::models::KeyType;
use crate::schema::users;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Identifiable, Queryable, AsChangeset)]
pub struct User {
    pub id: KeyType,

    pub username: String,
    pub password_hash: Option<String>,
    pub password_salt: Option<String>,

    pub email: String,
    pub is_active: bool,

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_photo: Option<String>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
}
