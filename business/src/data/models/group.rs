use crate::models::validate::RE_PACKAGE;
use crate::models::KeyType;
use crate::schema::groups;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(
    AsChangeset,
    Clone,
    Debug,
    Deserialize,
    Identifiable,
    Queryable,
    Serialize,
    Validate,
)]
pub struct Group {
    #[serde(skip)]
    pub id: KeyType,

    #[validate(regex = "RE_PACKAGE")]
    pub name: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "groups"]
pub struct NewGroup {
    pub name: String,
}
