use crate::models::KeyType;
use crate::schema::products;

use serde::{Deserialize, Serialize};

#[derive(
    AsChangeset, Clone, Debug, Deserialize, Identifiable, Queryable, Serialize,
)]
pub struct Product {
    #[serde(skip)]
    pub id: KeyType,
    pub title: String,
    pub description: String,
    pub available_quantity: i32,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "products"]
pub struct NewProduct {
    pub title: String,
    pub description: String,
    pub available_quantity: i32,
}
