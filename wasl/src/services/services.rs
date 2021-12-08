use crate::result::Result;
use data::models::NewService;

use data::connection::*;
use data::diesel::prelude::*;
use data::models::{KeyType, Service};

pub struct ServicesService {
    conn: PostgresConnection,
}

impl ServicesService {
    pub fn new(conn: PostgresConnection) -> ServicesService {
        ServicesService { conn }
    }

    pub fn get_service_by_id(&self, key: KeyType) -> Result<Service> {
        use data::schema::services::dsl::*;

        // TODO:
        // Properly handle NotFound error

        Ok(data::result::adapt(
            services.find(key).get_result(&self.conn.get()?),
        )?)
    }

    pub fn create(&self, new_service: &NewService) -> Result<Service> {
        use data::schema::services::dsl::*;

        Ok(data::result::adapt(
            data::diesel::insert_into(services)
                .values(new_service)
                .get_result(&self.conn.get()?),
        )?)
    }
}
