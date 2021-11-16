use crate::models::group::*;
use crate::repos::DbPool;
use crate::result::{DataError, Result};

use data_derive::Repository;
use diesel::prelude::*;

#[derive(Clone, Repository)]
#[repo_table_name = "groups"]
#[repo_model = "Group"]
#[repo_insert_model = "NewGroup"]
pub struct GroupsRepo {
    pub pool: DbPool,
}
