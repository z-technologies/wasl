use crate::models::{KeyType, User};
use crate::schema::confirmations;

use chrono::NaiveDateTime;
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
    pub user_id: KeyType,
    pub otp: String,
    pub token: String,
    pub issued_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize, Validate)]
#[table_name = "confirmations"]
pub struct NewConfirmation {
    pub user_id: KeyType,
    pub otp: String,
    pub token: String,
    pub expires_at: NaiveDateTime,
}
