use crate::data::models::{KeyType, User};
use crate::data::schema::services;

use serde::{Deserialize, Serialize};

#[derive(
    AsChangeset,
    Associations,
    Clone,
    Debug,
    Deserialize,
    Identifiable,
    Queryable,
    Serialize,
)]
#[belongs_to(User)]
pub struct Service {
    #[serde(skip)]
    pub id: KeyType,
    pub title: String,
    pub description: String,
    pub available_begin: Option<chrono::NaiveTime>,
    pub available_end: Option<chrono::NaiveTime>,
    pub user_id: KeyType,
}

#[derive(Debug, Insertable)]
#[table_name = "services"]
pub struct NewService {
    pub title: String,
    pub description: String,
    pub available_begin: Option<chrono::NaiveTime>,
    pub available_end: Option<chrono::NaiveTime>,
    pub user_id: KeyType,
}
