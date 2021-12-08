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
        // Rethink option

        Ok(data::result::adapt(
            confirmations
                .filter(token.eq(t))
                .first::<Confirmation>(&self.conn.get()?)
                .optional(),
        )?)
    }

    pub fn get_by_otp(&self, o: &str) -> Result<Option<Confirmation>> {
        use data::schema::confirmations::dsl::*;

        // TODO:
        // Rethink option

        Ok(data::result::adapt(
            confirmations
                .filter(otp.eq(o))
                .first::<Confirmation>(&self.conn.get()?)
                .optional(),
        )?)
    }

    pub fn create(&self, new_conf: &NewConfirmation) -> Result<Confirmation> {
        use data::schema::confirmations::dsl::*;

        Ok(data::result::adapt(
            data::diesel::insert_into(confirmations)
                .values(new_conf)
                .get_result(&self.conn.get()?),
        )?)
    }

    pub fn delete(&self, conf: Confirmation) -> Result<usize> {
        Ok(data::result::adapt(
            data::diesel::delete(&conf).execute(&self.conn.get()?),
        )?)
    }
}
