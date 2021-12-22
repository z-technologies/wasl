use crate::data::connection::*;
use crate::data::models::{
    FinancialRecord, FinancialRecordVerification,
    FinancialRecordVerificationOutcome, User,
};
use crate::result::Result;

use bigdecimal::BigDecimal;
use diesel::prelude::*;

pub struct FinancialRecordsService {
    conn: PostgresConnection,
}

impl FinancialRecordsService {
    pub fn new(conn: PostgresConnection) -> FinancialRecordsService {
        FinancialRecordsService { conn }
    }

    pub fn made_by(&self, user: &User) -> Result<Vec<FinancialRecord>> {
        Ok(FinancialRecord::belonging_to(user).load(&self.conn.get()?)?)
    }

    pub fn made_by_has_verification(
        &self,
        user: &User,
    ) -> Result<Vec<(FinancialRecordVerification, FinancialRecord)>> {
        use crate::data::schema::financial_record_verifications::dsl::*;
        use crate::data::schema::financial_records::dsl::*;

        Ok(financial_record_verifications
            .inner_join(financial_records)
            .filter(made_by.eq(user.id))
            .load(&self.conn.get()?)?)
    }

    /// TODO:
    /// Reimplement this to guarantee that the user cannot spend
    /// money that he/she requested to withdraw
    pub fn total_verified(&self, user: &User) -> Result<BigDecimal> {
        use crate::data::schema::financial_record_verifications::dsl::*;
        use crate::data::schema::financial_records::dsl::*;

        Ok(financial_record_verifications
            .inner_join(financial_records)
            .filter(made_by.eq(user.id))
            .filter(outcome.eq(FinancialRecordVerificationOutcome::Verified))
            .select(amount)
            .load(&self.conn.get()?)?
            .iter()
            .sum::<BigDecimal>())
    }
}
