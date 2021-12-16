use crate::data::models::{KeyType, User};
use crate::data::schema::{service_reservations, services};

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
pub struct ServiceReservation {
    #[serde(skip)]
    pub id: KeyType,
    pub made_by: KeyType,
    pub service_id: KeyType,
    pub reservation_begin: chrono::NaiveDateTime,
    pub reservation_end: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "service_reservations"]
pub struct NewServiceReservation {
    pub made_by: KeyType,
    pub service_id: KeyType,
    pub reservation_begin: chrono::NaiveDateTime,
    pub reservation_end: chrono::NaiveDateTime,
}
