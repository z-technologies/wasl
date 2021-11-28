use crate::models::KeyType;
use crate::schema::services;

use serde::{Deserialize, Serialize};

#[derive(
    AsChangeset, Clone, Debug, Deserialize, Identifiable, Queryable, Serialize,
)]
pub struct Service {
    #[serde(skip)]
    pub id: KeyType,
    pub title: String,
    pub description: String,
    pub available_begin: Option<chrono::NaiveTime>,
    pub available_end: Option<chrono::NaiveTime>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "services"]
pub struct NewService {
    pub title: String,
    pub description: String,
    pub available_begin: Option<chrono::NaiveTime>,
    pub available_end: Option<chrono::NaiveTime>,
}
