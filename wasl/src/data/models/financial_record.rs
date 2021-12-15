use crate::data::models::{KeyType, User};
use crate::data::schema::financial_records;

use bigdecimal::BigDecimal;
use validator::Validate;

#[derive(Associations, Clone, Debug, Identifiable, Queryable, Validate)]
#[belongs_to(User, foreign_key = "made_by")]
pub struct FinancialRecord {
    pub id: KeyType,

    pub amount: BigDecimal,
    pub state: Option<bool>,
    pub signature: String,

    pub made_by: KeyType,
    pub made_at: chrono::DateTime<chrono::Utc>,
}
