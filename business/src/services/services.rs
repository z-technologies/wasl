use crate::result::Result;

use data::context::DbContext;
use data::models::{KeyType, Service};
use data::repos::Repo;

pub struct ServicesService {
    ctx: DbContext,
}

impl ServicesService {
    pub fn new(ctx: DbContext) -> ServicesService {
        ServicesService { ctx }
    }

    pub fn get_service_by_id(&self, id: KeyType) -> Result<Service> {
        Ok(self.ctx.services().get(id)?)
    }
}
