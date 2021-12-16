use crate::data::models::User;
use crate::result::Result;
use crate::services::{FinancialRecordsService, TransactionsService};

use bigdecimal::BigDecimal;

use std::sync::Arc;

pub struct FinanceService {
    financial_records_svc: Arc<FinancialRecordsService>,
    transactions_svc: Arc<TransactionsService>,
}

impl FinanceService {
    pub fn new(
        financial_records_svc: Arc<FinancialRecordsService>,
        transactions_svc: Arc<TransactionsService>,
    ) -> FinanceService {
        FinanceService {
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
}
