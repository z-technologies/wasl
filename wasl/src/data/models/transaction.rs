use crate::data::models::{KeyType, User};
use crate::data::schema::transactions;

use bigdecimal::BigDecimal;

#[derive(Associations, Clone, Debug, Identifiable, Queryable)]
pub struct Transaction {
    pub id: KeyType,
    pub amount: BigDecimal,
    pub state: TransactionState,
    pub sender: KeyType,
    pub receiver: KeyType,
    pub made_at: chrono::NaiveTime,
}

#[derive(Debug, Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub amount: BigDecimal,
    pub state: TransactionState,
    pub sender: KeyType,
    pub receiver: KeyType,
}

#[derive(Clone, Debug, DbEnum)]
#[DieselType = "Transaction_state"]
pub enum TransactionState {
    Pending,
    Declined,
    Confirmed,
}

impl NewTransaction {
    pub fn new_pending(
        from: &User,
        to: &User,
        amount: BigDecimal,
    ) -> NewTransaction {
        NewTransaction {
            amount,
            state: TransactionState::Pending,
            sender: from.id,
            receiver: to.id,
        }
    }
}
