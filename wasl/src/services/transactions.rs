use crate::data::connection::*;
use crate::data::models::{
    NewTransaction, Transaction, TransactionConfirmationOutcome, User,
};
use crate::result::Result;

use bigdecimal::BigDecimal;
use diesel::prelude::*;

pub struct TransactionsService {
    conn: PostgresConnection,
}

impl TransactionsService {
    pub fn new(conn: PostgresConnection) -> TransactionsService {
        TransactionsService { conn }
    }

    pub fn sent_by(&self, user: &User) -> Result<Vec<Transaction>> {
        use crate::data::schema::transactions::dsl::*;

        Ok(transactions
            .filter(sender.eq(user.id))
            .load(&self.conn.get()?)?)
    }

    pub fn received_by(&self, user: &User) -> Result<Vec<Transaction>> {
        use crate::data::schema::transactions::dsl::*;

        Ok(transactions
            .filter(receiver.eq(user.id))
            .load(&self.conn.get()?)?)
    }

    pub fn total_in(&self, user: &User) -> Result<BigDecimal> {
        use crate::data::schema::transactions::dsl::*;

        Ok(transactions
            .select(amount)
            .filter(receiver.eq(user.id))
            .load(&self.conn.get()?)?
            .iter()
            .sum::<BigDecimal>())
    }

    pub fn total_in_confirmed(&self, user: &User) -> Result<BigDecimal> {
        use crate::data::schema::transaction_confirmations::dsl::*;
        use crate::data::schema::transactions::dsl::*;

        Ok(transaction_confirmations
            .inner_join(transactions)
            .select(amount)
            .filter(receiver.eq(user.id))
            .filter(outcome.eq(TransactionConfirmationOutcome::Confirmed))
            .load(&self.conn.get()?)?
            .iter()
            .sum::<BigDecimal>())
    }

    pub fn total_out(&self, user: &User) -> Result<BigDecimal> {
        use crate::data::schema::transactions::dsl::*;

        Ok(transactions
            .select(amount)
            .filter(sender.eq(user.id))
            .load(&self.conn.get()?)?
            .iter()
            .sum::<BigDecimal>())
    }

    pub fn total_out_confirmed(&self, user: &User) -> Result<BigDecimal> {
        use crate::data::schema::transaction_confirmations::dsl::*;
        use crate::data::schema::transactions::dsl::*;

        Ok(transaction_confirmations
            .inner_join(transactions)
            .select(amount)
            .filter(sender.eq(user.id))
            .filter(outcome.eq(TransactionConfirmationOutcome::Confirmed))
            .load(&self.conn.get()?)?
            .iter()
            .sum::<BigDecimal>())
    }

    pub fn total(&self, user: &User) -> Result<BigDecimal> {
        let total_in = self.total_in_confirmed(user)?;
        let total_out = self.total_out_confirmed(user)?;

        Ok(total_in - total_out)
    }

    pub fn total_usable(&self, user: &User) -> Result<BigDecimal> {
        let total_in = self.total_in_confirmed(user)?;
        let total_out = self.total_out(user)?;

        Ok(total_in - total_out)
    }

    pub fn create(
        &self,
        new_transaction: &NewTransaction,
    ) -> Result<Transaction> {
        use crate::data::schema::transactions::dsl::*;

        Ok(diesel::insert_into(transactions)
            .values(new_transaction)
            .get_result(&self.conn.get()?)?)
    }
}
