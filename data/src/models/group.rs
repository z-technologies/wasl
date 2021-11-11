use crate::models::KeyType;
use crate::schema::groups;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Identifiable, Queryable, AsChangeset)]
pub struct Group {
    pub id: KeyType,
    pub name: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "groups"]
pub struct NewGroup {
    pub name: String,
}
