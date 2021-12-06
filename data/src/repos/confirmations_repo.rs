use crate::models::Confirmation;
use crate::repos::{DbPool, DbPooledConnection, Repo};
use crate::result::Result;

use diesel::prelude::*;

#[derive(Clone)]
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

impl Repo<Confirmation> for ConfirmationsRepo {
    fn get_connection(&self) -> Result<DbPooledConnection> {
        Ok(self.pool.get()?)
    }
}
