use crate::data::connection::*;
use crate::data::models::{Transaction, TransactionState, User};
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

    pub fn total_sender_pending(&self, user: &User) -> Result<BigDecimal> {
        use crate::data::schema::transactions::dsl::*;

        Ok(transactions
            .select(amount)
            .filter(sender.eq(user.id))
            .filter(state.eq(TransactionState::Pending))
            .load(&self.conn.get()?)?
            .iter()
            .sum::<BigDecimal>())
    }

    pub fn total_receiver_pending(&self, user: &User) -> Result<BigDecimal> {
        use crate::data::schema::transactions::dsl::*;

        Ok(transactions
            .select(amount)
            .filter(receiver.eq(user.id))
            .filter(state.eq(TransactionState::Pending))
            .load(&self.conn.get()?)?
            .iter()
            .sum::<BigDecimal>())
    }

    pub fn total_sender_confirmed(&self, user: &User) -> Result<BigDecimal> {
        use crate::data::schema::transactions::dsl::*;

        Ok(transactions
            .select(amount)
            .filter(sender.eq(user.id))
            .filter(state.eq(TransactionState::Confirmed))
            .load(&self.conn.get()?)?
            .iter()
            .sum::<BigDecimal>())
    }

    pub fn total_receiver_confirmed(&self, user: &User) -> Result<BigDecimal> {
        use crate::data::schema::transactions::dsl::*;

        Ok(transactions
            .select(amount)
            .filter(receiver.eq(user.id))
            .filter(state.eq(TransactionState::Confirmed))
            .load(&self.conn.get()?)?
            .iter()
            .sum::<BigDecimal>())
    }
}
