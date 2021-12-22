use crate::data::models::{KeyType, User};
use crate::data::schema::{transaction_confirmations, transactions};

use bigdecimal::BigDecimal;

#[derive(Associations, Clone, Debug, Identifiable, Queryable)]
pub struct Transaction {
    pub id: KeyType,
    pub amount: BigDecimal,
    pub confirmation_token: String,
    pub sender: KeyType,
    pub receiver: KeyType,
    pub made_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub amount: BigDecimal,
    pub confirmation_token: String,
    pub sender: KeyType,
    pub receiver: KeyType,
}

impl NewTransaction {
    pub fn new(from: &User, to: &User, amount: BigDecimal) -> NewTransaction {
        NewTransaction {
            amount,
            sender: from.id,
            receiver: to.id,
        }
    }
}

#[derive(Associations, Clone, Debug, Identifiable, Queryable)]
#[belongs_to(Transaction)]
pub struct TransactionConfirmation {
    pub id: KeyType,
    pub outcome: TransactionConfirmationOutcome,
    pub transaction_id: KeyType,
    pub confirmed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Insertable)]
#[table_name = "transaction_confirmations"]
pub struct NewTransactionConfirmation {
    pub outcome: TransactionConfirmationOutcome,
    pub transaction_id: KeyType,
}

#[derive(Clone, Debug, DbEnum)]
#[DieselType = "Transaction_confirmation_outcome"]
pub enum TransactionConfirmationOutcome {
    Declined,
    Confirmed,
}

impl NewTransactionConfirmation {
    pub fn new(
        outcome: TransactionConfirmationOutcome,
        transaction: &Transaction,
    ) -> NewTransactionConfirmation {
        NewTransactionConfirmation {
            outcome,
            transaction_id: transaction.id,
        }
    }
}
