use crate::result::Result;

use data::context::*;
use data::diesel::prelude::*;
use data::models::{KeyType, Service};

pub struct ServicesService {
    conn: PostgresConnection,
}

impl ServicesService {
    pub fn new(conn: PostgresConnection) -> ServicesService {
        ServicesService { conn }
    }

    pub fn get_service_by_id(&self, id: KeyType) -> Result<Service> {
        use data::schema::services::dsl::*;

        // TODO:
        // Properly handle errors

        Ok(services.find(id).get_result(&self.conn.get()?).unwrap())
    }
}
