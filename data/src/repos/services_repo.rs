use crate::models::service::*;
use crate::repos::DbPool;
use crate::result::{DataError, Result};

use data_derive::Repository;
use diesel::prelude::*;

#[derive(Clone, Repository)]
#[repo_table_name = "services"]
#[repo_model = "Service"]
#[repo_insert_model = "NewService"]
pub struct ServicesRepo {
    pub pool: DbPool,
}
