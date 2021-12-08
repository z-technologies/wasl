use crate::data::models::validate::RE_USERNAME;
use crate::data::models::{Group, KeyType};
use crate::data::schema::{user_groups, users};

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(
    AsChangeset,
    Clone,
    Debug,
    Deserialize,
    Identifiable,
    Queryable,
    Serialize,
    Validate,
)]
pub struct User {
    #[serde(skip)]
    pub id: KeyType,

    #[validate(regex = "RE_USERNAME")]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[serde(skip)]
    pub password_hash: String,
    #[serde(skip)]
    pub is_active: bool,

    #[validate(length(min = 2, max = 32))]
    pub first_name: Option<String>,
    #[validate(length(min = 2, max = 32))]
    pub last_name: Option<String>,
    #[validate(url)]
    pub profile_photo: Option<String>,

    pub cached_balance: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Insertable, Serialize, Deserialize, Validate)]
#[table_name = "users"]
pub struct NewUser {
    #[validate(regex = "RE_USERNAME")]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password_hash: String,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(User)]
#[belongs_to(Group)]
pub struct UserGroup {
    pub id: KeyType,
    pub user_id: KeyType,
    pub group_id: KeyType,
}
