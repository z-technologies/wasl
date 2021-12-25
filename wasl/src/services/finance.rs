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
        let from_transactions = self.transactions_svc.as_ref().total(user)?;

        Ok(from_financial_recrods + from_transactions)
    }

    pub fn usable_balance(&self, user: &User) -> Result<BigDecimal> {
        let from_financial_recrods =
            self.financial_records_svc.as_ref().total_verified(user)?;
        let from_transactions =
            self.transactions_svc.as_ref().total_usable(user)?;

        Ok(from_financial_recrods + from_transactions)
    }

    pub fn transfer(
        &self,
        from: &User,
        to: &User,
        amount: BigDecimal,
        private_key: &[u8],
    ) -> Result<Transaction> {
        Ok(self
            .conn
            .get()?
            .build_transaction()
            .run::<Transaction, UserError, _>(|| {
                if amount > self.usable_balance(from)? {
                    return Err(UserError::InsufficientBalance);
                }

                Ok(self.transactions_svc.as_ref().create(
                    &NewTransaction::new(from, to, amount, private_key)?,
                )?)
            })?)
    }

    pub fn financial_records_service(&self) -> &Arc<FinancialRecordsService> {
        &self.financial_records_svc
    }

    pub fn transactions_service(&self) -> &Arc<TransactionsService> {
        &self.transactions_svc
    }
}
