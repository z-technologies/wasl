use crate::models::validate::RE_USERNAME;
use crate::models::KeyType;
use crate::schema::users;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Identifiable, Queryable, AsChangeset, Validate)]
pub struct User {
    pub id: KeyType,

    #[validate(regex = "RE_USERNAME")]
    pub username: String,
    pub password_hash: Option<String>,
    pub password_salt: Option<String>,

    #[validate(email)]
    pub email: String,
    pub is_active: bool,

    #[validate(length(min = 2, max = 32))]
    pub first_name: Option<String>,
    #[validate(length(min = 2, max = 32))]
    pub last_name: Option<String>,

    pub profile_photo: Option<String>,
    pub is_provider: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize, Validate)]
#[table_name = "users"]
pub struct NewUser {
    //#[validate(regex = "RE_USERNAME")]
    pub username: String,
    //#[validate(email)]
    pub email: String,
    //#[validate(length(min = 2, max = 32))]
    pub first_name: Option<String>,
    //#[validate(length(min = 2, max = 32))]
    pub last_name: Option<String>,
    pub is_provider: Option<bool>,
}
