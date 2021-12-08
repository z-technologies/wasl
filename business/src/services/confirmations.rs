use crate::result::{Result, UserError};
use crate::security::random::generate_alphanum_string;

use data::connection::*;
use data::diesel::prelude::*;
use data::models::{Confirmation, NewConfirmation, User};

pub struct ConfirmationsService {
    conn: PostgresConnection,
}

impl ConfirmationsService {
    pub fn new(conn: PostgresConnection) -> ConfirmationsService {
        ConfirmationsService { conn }
    }

    pub fn get_by_token(&self, t: &str) -> Result<Confirmation> {
        use data::schema::confirmations::dsl::*;

        data::result::adapt(
            confirmations
                .filter(token.eq(t))
                .first::<Confirmation>(&self.conn.get()?)
                .optional(),
        )?
        .ok_or(UserError::InvalidConfirmationDetails)
    }

    pub fn get_by_otp(&self, o: &str) -> Result<Confirmation> {
        use data::schema::confirmations::dsl::*;

        data::result::adapt(
            confirmations
                .filter(otp.eq(o))
                .first::<Confirmation>(&self.conn.get()?)
                .optional(),
        )?
        .ok_or(UserError::InvalidConfirmationDetails)
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

    pub fn generate_for<const OTP_LEN: usize, const TOKEN_LEN: usize>(
        &self,
        user: &User,
        valid_for: chrono::Duration,
    ) -> Result<Confirmation> {
        let conf = NewConfirmation {
            user_id: user.id,
            otp: generate_alphanum_string::<OTP_LEN>(),
            token: generate_alphanum_string::<TOKEN_LEN>(),
            issued_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + valid_for,
        };

        Ok(self.create(&conf)?)
    }
}
