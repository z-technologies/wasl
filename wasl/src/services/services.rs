use crate::data::connection::*;
use crate::data::models::{
    KeyType, NewService, NewServiceReservation, Service, ServiceReservation,
    Transaction, User,
};
use crate::result::{Result, UserError};
use crate::services::{FinanceService, UsersService};

use diesel::prelude::*;

use std::sync::Arc;

pub struct ServicesService {
    conn: PostgresConnection,
    users_svc: Arc<UsersService>,
    finance_svc: Arc<FinanceService>,
}

impl ServicesService {
    pub fn new(
        conn: PostgresConnection,
        users_svc: Arc<UsersService>,
        finance_svc: Arc<FinanceService>,
    ) -> ServicesService {
        ServicesService {
            conn,
            users_svc,
            finance_svc,
        }
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

    pub fn is_available(
        &self,
        service: &Service,
        begin: &chrono::NaiveDateTime,
        end: &chrono::NaiveDateTime,
    ) -> Result<bool> {
        let reservations = self.reservations(service)?;

        Ok(reservations.iter().all(|r| {
            !periods_overlap(
                (&r.reservation_begin, &r.reservation_end),
                (begin, end),
            )
        }))
    }

    pub fn make_reservation(
        &self,
        service: &Service,
        customer: &User,
        begin: chrono::NaiveDateTime,
        end: chrono::NaiveDateTime,
    ) -> Result<(ServiceReservation, Transaction)> {
        use crate::data::schema::service_reservations::dsl::*;

        Ok(self
            .conn
            .get()?
            .build_transaction()
            .run::<_, UserError, _>(|| {
                if !self.is_available(&service, &begin, &end)? {
                    return Err(UserError::TimePeriodsOverlap);
                }

                let new_reservation =
                    NewServiceReservation::new(customer, service, begin, end);
                let transaction = self.finance_svc.transfer_pending(
                    customer,
                    &self.users_svc.get_by_id(service.id)?,
                    service.price.clone(),
                )?;

                Ok((
                    diesel::insert_into(service_reservations)
                        .values(&new_reservation)
                        .get_result(&self.conn.get()?)?,
                    transaction,
                ))
            })?)
    }
}

#[inline]
fn periods_overlap(
    lhs: (&chrono::NaiveDateTime, &chrono::NaiveDateTime),
    rhs: (&chrono::NaiveDateTime, &chrono::NaiveDateTime),
) -> bool {
    lhs.0 < rhs.1 && rhs.0 < lhs.1
}
