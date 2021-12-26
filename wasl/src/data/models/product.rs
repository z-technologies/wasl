use crate::data::models::{KeyType, Transaction, User};
use crate::data::schema::{product_orders, products};

use bigdecimal::BigDecimal;
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
    pub price: BigDecimal,
    pub available_quantity: i64,
    pub user_id: KeyType,
}

#[derive(Debug, Insertable)]
#[table_name = "products"]
pub struct NewProduct {
    pub title: String,
    pub description: String,
    pub price: BigDecimal,
    pub available_quantity: i64,
    pub user_id: KeyType,
}

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
#[belongs_to(User, foreign_key = made_by)]
#[belongs_to(Product)]
pub struct ProductOrder {
    #[serde(skip)]
    pub id: KeyType,
    pub state: ProductOrderState,
    pub quantity: i64,
    pub product_id: KeyType,
    pub transaction_id: KeyType,
    pub made_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Insertable)]
#[table_name = "product_orders"]
pub struct NewProductOrder {
    pub state: ProductOrderState,
    pub quantity: i64,
    pub product_id: KeyType,
    pub transaction_id: KeyType,
}

#[derive(Clone, Debug, DbEnum, Serialize, Deserialize)]
#[DieselType = "Product_order_state"]
pub enum ProductOrderState {
    Pending,
    Declined,
    Accepted,
}

impl NewProductOrder {
    pub fn new(
        product: &Product,
        quantity: i64,
        transaction: &Transaction,
    ) -> Self {
        Self {
            state: ProductOrderState::Pending,
            quantity,
            product_id: product.id,
            transaction_id: transaction.id,
        }
    }
}
