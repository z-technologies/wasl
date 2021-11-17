use crate::models::confirmation::*;
use crate::repos::DbPool;
use crate::result::{DataError, Result};

use data_derive::Repository;
use diesel::prelude::*;

#[derive(Clone, Repository)]
#[repo_table_name = "confirmations"]
#[repo_model = "Confirmation"]
#[repo_insert_model = "NewConfirmation"]
pub struct ConfirmationsRepo {
    pub pool: DbPool,
}
