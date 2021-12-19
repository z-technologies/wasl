use crate::data::models::{KeyType, User};
use crate::data::schema::transactions;

use bigdecimal::BigDecimal;

#[derive(Associations, Clone, Debug, Identifiable, Queryable)]
pub struct Transaction {
    pub id: KeyType,
    pub amount: BigDecimal,
    pub sender: KeyType,
    pub receiver: KeyType,
    pub made_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub amount: BigDecimal,
    pub sender: KeyType,
    pub receiver: KeyType,
}

impl NewTransaction {
    pub fn new_pending(
        from: &User,
        to: &User,
        amount: BigDecimal,
    ) -> NewTransaction {
        NewTransaction {
            amount,
            sender: from.id,
            receiver: to.id,
        }
    }
}
