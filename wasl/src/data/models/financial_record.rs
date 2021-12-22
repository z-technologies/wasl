use crate::data::models::{KeyType, User};
use crate::data::schema::{financial_record_verifications, financial_records};

use bigdecimal::BigDecimal;

#[derive(Associations, Clone, Debug, Identifiable, Queryable)]
#[belongs_to(User, foreign_key = "made_by")]
pub struct FinancialRecord {
    pub id: KeyType,
    pub amount: BigDecimal,
    pub made_by: KeyType,
    pub made_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Associations, Clone, Debug, Identifiable, Queryable)]
#[belongs_to(User, foreign_key = "verified_by")]
#[belongs_to(FinancialRecord)]
pub struct FinancialRecordVerification {
    pub id: KeyType,
    pub outcome: FinancialRecordVerificationOutcome,
    pub financial_record_id: KeyType,
    pub verified_at: chrono::NaiveDateTime,
    pub verified_by: KeyType,
}

#[derive(Clone, Debug, DbEnum)]
#[DieselType = "Financial_record_verification_outcome"]
pub enum FinancialRecordVerificationOutcome {
    Rejected,
    Verified,
}
