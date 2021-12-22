use crate::data::connection::*;
use crate::data::models::{NewTransaction, Transaction, User};
use crate::result::{Result, UserError};
use crate::services::{FinancialRecordsService, TransactionsService};

use bigdecimal::BigDecimal;

use std::sync::Arc;

pub struct FinanceService {
    conn: PostgresConnection,
    financial_records_svc: Arc<FinancialRecordsService>,
    transactions_svc: Arc<TransactionsService>,
}

impl FinanceService {
    pub fn new(
        conn: PostgresConnection,
        financial_records_svc: Arc<FinancialRecordsService>,
        transactions_svc: Arc<TransactionsService>,
    ) -> FinanceService {
        FinanceService {
            conn,
            financial_records_svc,
            transactions_svc,
        }
    }

    pub fn virtual_balance(&self, user: &User) -> Result<BigDecimal> {
        let from_financial_recrods =
            self.financial_records_svc.as_ref().total_verified(user)?;

        let from_transactions_in = self
            .transactions_svc
            .as_ref()
            .total_receiver_confirmed(user)?;

        let from_transactions_out = self
            .transactions_svc
            .as_ref()
            .total_sender_confirmed(user)?;

        Ok(from_financial_recrods + from_transactions_in
            - from_transactions_out)
    }

    pub fn usable_balance(&self, user: &User) -> Result<BigDecimal> {
        let from_financial_recrods =
            self.financial_records_svc.as_ref().total_verified(user)?;

        let from_transactions_in = self
            .transactions_svc
            .as_ref()
            .total_receiver_confirmed(user)?;

        let from_transactions_out = self
            .transactions_svc
            .as_ref()
            .total_sender_confirmed(user)?;

        Ok(from_financial_recrods + from_transactions_in
            - from_transactions_out)
    }

    pub fn transfer_pending(
        &self,
        from: &User,
        to: &User,
        amount: BigDecimal,
    ) -> Result<Transaction> {
        Ok(self
            .conn
            .get()?
            .build_transaction()
            .run::<Transaction, UserError, _>(|| {
                if amount < self.usable_balance(from)? {
                    return Err(UserError::InsufficientBalance);
                }

                let new_transaction =
                    NewTransaction::new_pending(from, to, amount);

                Ok(self.transactions_svc.as_ref().create(&new_transaction)?)
            })?)
    }
}