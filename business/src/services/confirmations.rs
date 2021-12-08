use crate::result::Result;

use data::connection::*;
use data::diesel::prelude::*;
use data::models::{Confirmation, NewConfirmation};

pub struct ConfirmationsService {
    conn: PostgresConnection,
}

impl ConfirmationsService {
    pub fn new(conn: PostgresConnection) -> ConfirmationsService {
        ConfirmationsService { conn }
    }

    pub fn get_by_token(&self, t: &str) -> Result<Option<Confirmation>> {
        use data::schema::confirmations::dsl::*;

        // TODO:
        // Properly handle errors

        Ok(confirmations
            .filter(token.eq(t))
            .first::<Confirmation>(&self.conn.get()?)
            .optional()
            .unwrap())
    }

    pub fn get_by_otp(&self, o: &str) -> Result<Option<Confirmation>> {
        use data::schema::confirmations::dsl::*;

        // TODO:
        // Properly handle errors

        Ok(confirmations
            .filter(otp.eq(o))
            .first::<Confirmation>(&self.conn.get()?)
            .optional()
            .unwrap())
    }

    pub fn create(&self, new_conf: &NewConfirmation) -> Result<Confirmation> {
        use data::schema::confirmations::dsl::*;

        // TODO:
        // Properly handle errors

        Ok(data::diesel::insert_into(confirmations)
            .values(new_conf)
            .get_result(&self.conn.get()?)
            .unwrap())
    }

    pub fn delete(&self, conf: Confirmation) -> Result<usize> {
        // TODO:
        // Properly handle errors

        Ok(data::diesel::delete(&conf)
            .execute(&self.conn.get()?)
            .unwrap())
    }
}
