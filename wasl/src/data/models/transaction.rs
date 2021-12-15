use crate::data::models::KeyType;
use crate::data::schema::transactions;

use bigdecimal::BigDecimal;

#[derive(Associations, Clone, Debug, Identifiable, Queryable)]
pub struct Transaction {
    pub id: KeyType,

    pub amount: BigDecimal,
    pub state: Option<bool>,
    pub signature: String,

    pub sender: KeyType,
    pub receiver: KeyType,

    pub made_at: chrono::DateTime<chrono::Utc>,
}
