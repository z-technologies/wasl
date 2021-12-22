use crate::data::models::{KeyType, User};
use crate::data::schema::confirmations;

use diesel::Identifiable;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(
    AsChangeset,
    Associations,
    Clone,
    Debug,
    Deserialize,
    Identifiable,
    Queryable,
    Serialize,
    Validate,
)]
#[belongs_to(User)]
pub struct Confirmation {
    #[serde(skip)]
    pub id: KeyType,
    pub otp: String,
    pub token: String,
    pub user_id: KeyType,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Insertable, Serialize, Deserialize, Validate)]
#[table_name = "confirmations"]
pub struct NewConfirmation {
    pub otp: String,
    pub token: String,
    pub user_id: KeyType,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}
