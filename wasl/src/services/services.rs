use crate::data::connection::*;
use crate::data::models::TransactionConfirmation;
use crate::data::models::{
    KeyType, NewService, NewServiceReservation, Service, ServiceReservation,
    Transaction, TransactionConfirmationOutcome, User,
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

    pub fn get(&self, key: KeyType) -> Result<Service> {
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
        begin: &chrono::DateTime<chrono::Utc>,
        end: &chrono::DateTime<chrono::Utc>,
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
        begin: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
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

                let transaction = self.finance_svc.transfer(
                    customer,
                    &self.users_svc.get_by_id(service.id)?,
                    service.price.clone(),
                )?;

                Ok((
                    diesel::insert_into(service_reservations)
                        .values(&NewServiceReservation::new(
                            begin,
                            end,
                            service,
                            &transaction,
                        ))
                        .get_result(&self.conn.get()?)?,
                    transaction,
                ))
            })?)
    }

    pub fn confirm_reservation(
        &self,
        reservation: &ServiceReservation,
    ) -> Result<TransactionConfirmation> {
        self.finalize_reservation(
            reservation,
            TransactionConfirmationOutcome::Confirmed,
        )
    }

    pub fn decline_reservation(
        &self,
        reservation: &ServiceReservation,
    ) -> Result<TransactionConfirmation> {
        self.finalize_reservation(
            reservation,
            TransactionConfirmationOutcome::Declined,
        )
    }

    #[inline]
    fn finalize_reservation(
        &self,
        reservation: &ServiceReservation,
        outcome: TransactionConfirmationOutcome,
    ) -> Result<TransactionConfirmation> {
        use crate::data::schema::transactions::dsl::*;

        let transaction = transactions
            .find(reservation.transaction_id)
            .get_result(&self.conn.get()?)?;

        self.finance_svc
            .transactions_service()
            .confirm(&transaction, outcome)
    }
}

#[inline]
fn periods_overlap(
    lhs: (
        &chrono::DateTime<chrono::Utc>,
        &chrono::DateTime<chrono::Utc>,
    ),
    rhs: (
        &chrono::DateTime<chrono::Utc>,
        &chrono::DateTime<chrono::Utc>,
    ),
) -> bool {
    lhs.0 < rhs.1 && rhs.0 < lhs.1
}
