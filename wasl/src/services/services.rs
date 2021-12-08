use crate::data::connection::*;
use crate::data::models::{KeyType, NewService, Service};
use crate::result::Result;

use diesel::prelude::*;

pub struct ServicesService {
    conn: PostgresConnection,
}

impl ServicesService {
    pub fn new(conn: PostgresConnection) -> ServicesService {
        ServicesService { conn }
    }

    pub fn get_service_by_id(&self, key: KeyType) -> Result<Service> {
        use crate::data::schema::services::dsl::*;

        Ok(services.find(key).get_result(&self.conn.get()?)?)
    }

    pub fn create(&self, new_service: &NewService) -> Result<Service> {
        use crate::data::schema::services::dsl::*;

        Ok(diesel::insert_into(services)
            .values(new_service)
            .get_result(&self.conn.get()?)?)
    }
}
