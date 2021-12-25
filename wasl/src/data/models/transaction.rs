use crate::data::models::{KeyType, User};
use crate::data::schema::{transaction_confirmations, transactions};
use crate::result::{Result, UserError};
use crate::security::random::generate_alphanum_string;
use crate::security::signature::ECDSASignature;

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

impl Transaction {
    pub fn verify(&self, key: &[u8]) -> Result<()> {
        let parts = self.confirmation_token.split_once(".");

        if let Some((token, signature)) = parts {
            let signature_bytes = base64::decode(signature)?;

            return ECDSASignature::new(key)?
                .verify(token.as_bytes(), &signature_bytes[..]);
        }

        Err(UserError::InvalidConfirmationDetails)
    }
}

impl NewTransaction {
    pub fn new(
        from: &User,
        to: &User,
        amount: BigDecimal,
        key: &[u8],
    ) -> Result<NewTransaction> {
        let rand_str = generate_alphanum_string::<32>();
        let signature =
            ECDSASignature::new(key)?.sign_base64(&rand_str.as_bytes());

        Ok(NewTransaction {
            amount,
            confirmation_token: format!("{}.{}", rand_str, signature),
            sender: from.id,
            receiver: to.id,
        })
    }
}

impl NewTransactionConfirmation {
    pub fn new(
        outcome: TransactionConfirmationOutcome,
        transaction: &Transaction,
        key: &[u8],
    ) -> Result<NewTransactionConfirmation> {
        transaction.verify(key)?;

        Ok(NewTransactionConfirmation {
            outcome,
            transaction_id: transaction.id,
        })
    }
}
