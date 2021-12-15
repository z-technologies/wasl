use crate::data::models::{KeyType, User};
use crate::data::schema::financial_records;

use bigdecimal::BigDecimal;
use validator::Validate;

#[derive(Associations, Clone, Debug, Identifiable, Queryable, Validate)]
#[belongs_to(User, foreign_key = "made_by")]
pub struct FinancialRecord {
    pub id: KeyType,

    pub amount: BigDecimal,

    pub state: FinancialRecordState,
    pub signature: String,

    pub made_by: KeyType,
    pub made_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, DbEnum)]
#[DieselType = "Financial_record_state"]
pub enum FinancialRecordState {
    Pending,
    Verified,
    Rejected,
}
