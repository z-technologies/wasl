use crate::data::models::{KeyType, User};
use crate::data::schema::products;

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
pub struct Product {
    #[serde(skip)]
    pub id: KeyType,
    pub title: String,
    pub description: String,
    pub available_quantity: i32,
    pub user_id: KeyType,
}

#[derive(Debug, Insertable)]
#[table_name = "products"]
pub struct NewProduct {
    pub title: String,
    pub description: String,
    pub available_quantity: i32,
    pub user_id: KeyType,
}
