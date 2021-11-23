use crate::models::confirmation::*;
use crate::repos::{DbPool, Repo};
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

impl ConfirmationsRepo {
    pub fn get_by_token(&self, t: &str) -> Result<Option<Confirmation>> {
        use crate::schema::confirmations::dsl::*;

        Ok(confirmations
            .filter(token.eq(t))
            .first::<Confirmation>(&self.get_connection()?)
            .optional()?)
    }

    pub fn get_by_otp(&self, o: &str) -> Result<Option<Confirmation>> {
        use crate::schema::confirmations::dsl::*;

        Ok(confirmations
            .filter(otp.eq(o))
            .first::<Confirmation>(&self.get_connection()?)
            .optional()?)
    }
}
