use crate::data::models::{KeyType, Transaction, User};
use crate::data::schema::{service_reservations, services};

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
pub struct Service {
    #[serde(skip)]
    pub id: KeyType,
    pub title: String,
    pub description: String,
    pub price: BigDecimal,
    pub available_begin: Option<chrono::NaiveTime>,
    pub available_end: Option<chrono::NaiveTime>,
    pub user_id: KeyType,
}

#[derive(Debug, Insertable)]
#[table_name = "services"]
pub struct NewService {
    pub title: String,
    pub description: String,
    pub price: BigDecimal,
    pub available_begin: Option<chrono::NaiveTime>,
    pub available_end: Option<chrono::NaiveTime>,
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
#[belongs_to(Service)]
#[belongs_to(Transaction)]
pub struct ServiceReservation {
    #[serde(skip)]
    pub id: KeyType,
    pub reservation_begin: chrono::DateTime<chrono::Utc>,
    pub reservation_end: chrono::DateTime<chrono::Utc>,
    pub service_id: KeyType,
    pub transaction_id: KeyType,
}

#[derive(Debug, Insertable)]
#[table_name = "service_reservations"]
pub struct NewServiceReservation {
    pub reservation_begin: chrono::DateTime<chrono::Utc>,
    pub reservation_end: chrono::DateTime<chrono::Utc>,
    pub service_id: KeyType,
    pub transaction_id: KeyType,
}

impl NewServiceReservation {
    pub fn new(
        begin: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
        service: &Service,
        transaction: &Transaction,
    ) -> NewServiceReservation {
        NewServiceReservation {
            reservation_begin: begin,
            reservation_end: end,
            service_id: service.id,
            transaction_id: transaction.id,
        }
    }
}
