use crate::data::connection::*;
use crate::data::models::{KeyType, NewService, Service, User};
use crate::result::{Result, UserError};

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

    pub fn delete(&self, service: Service, for_user: &User) -> Result<usize> {
        if service.user_id != for_user.id {
            return Err(UserError::PermissionDenied);
        }

        Ok(diesel::delete(&service).execute(&self.conn.get()?)?)
    }
}
