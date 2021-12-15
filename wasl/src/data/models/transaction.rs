use crate::data::models::KeyType;
use crate::data::schema::transactions;

use bigdecimal::BigDecimal;

#[derive(Associations, Clone, Debug, Identifiable, Queryable)]
pub struct Transaction {
    pub id: KeyType,

    pub amount: BigDecimal,

    pub state: TransactionState,
    pub signature: String,

    pub sender: KeyType,
    pub receiver: KeyType,

    pub made_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, DbEnum)]
#[DieselType = "Transaction_state"]
pub enum TransactionState {
    Pending,
    Declined,
    Confirmed,
}
