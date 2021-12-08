use crate::result::Result;

use data::context::DbContext;
use data::models::{Confirmation, NewConfirmation};
use data::repos::Repo;

pub struct ConfirmationsService {
    ctx: DbContext,
}

impl ConfirmationsService {
    pub fn new(ctx: DbContext) -> ConfirmationsService {
        ConfirmationsService { ctx }
    }

    pub fn get_by_token(&self, token: &str) -> Result<Option<Confirmation>> {
        Ok(self.ctx.confirmations().get_by_token(token)?)
    }

    pub fn get_by_otp(&self, otp: &str) -> Result<Option<Confirmation>> {
        Ok(self.ctx.confirmations().get_by_otp(otp)?)
    }

    pub fn create(&self, new_conf: &NewConfirmation) -> Result<Confirmation> {
        Ok(self.ctx.confirmations().add(new_conf)?)
    }

    pub fn delete(&self, conf: Confirmation) -> Result<usize> {
        Ok(self.ctx.confirmations().remove(conf)?)
    }
}
