use crate::data::connection::*;
use crate::data::models::{
    KeyType, NewService, NewServiceReservation, Service, ServiceReservation,
    User,
};
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

    pub fn reservations(
        &self,
        service: &Service,
    ) -> Result<Vec<ServiceReservation>> {
        Ok(
            ServiceReservation::belonging_to(service)
                .load(&self.conn.get()?)?,
        )
    }

    pub fn can_make_reservation(
        &self,
        service: &Service,
        reservation: &NewServiceReservation,
    ) -> Result<bool> {
        let reservations = self.reservations(service)?;

        Ok(reservations.iter().all(|r| {
            !periods_overlap(
                (&r.reservation_begin, &r.reservation_end),
                (&reservation.reservation_begin, &reservation.reservation_end),
            )
        }))
    }

    pub fn make_reservation(
        &self,
        reservation: &NewServiceReservation,
    ) -> Result<ServiceReservation> {
        use crate::data::schema::service_reservations::dsl::*;
        use crate::data::schema::services::dsl::*;

        let service = services
            .find(reservation.service_id)
            .get_result(&self.conn.get()?)?;

        if !self.can_make_reservation(&service, reservation)? {
            return Err(UserError::TimePeriodsOverlap);
        }

        Ok(diesel::insert_into(service_reservations)
            .values(reservation)
            .get_result(&self.conn.get()?)?)
    }
}

#[inline]
fn periods_overlap(
    lhs: (&chrono::NaiveDateTime, &chrono::NaiveDateTime),
    rhs: (&chrono::NaiveDateTime, &chrono::NaiveDateTime),
) -> bool {
    lhs.0 < rhs.1 && rhs.0 < lhs.1
}
