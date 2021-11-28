use crate::models::product::*;
use crate::repos::DbPool;
use crate::result::{DataError, Result};

use data_derive::Repository;
use diesel::prelude::*;

#[derive(Clone, Repository)]
#[repo_table_name = "products"]
#[repo_model = "Product"]
#[repo_insert_model = "NewProduct"]
pub struct ProductsRepo {
    pub pool: DbPool,
}
