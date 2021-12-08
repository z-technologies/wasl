use crate::handlers::{auth, echo, services};
use crate::settings::Settings;

use business::services::*;
use data::connection::PostgresConnection;

use actix_web::web;

use std::env;
use std::sync::Arc;

pub fn setup_handlers(cfg: &mut web::ServiceConfig) {
    cfg.service({
        web::scope("/api/v1")
            .service(web::scope("/test").service(echo::echo))
            .service(
                web::scope("/auth")
                    .service(auth::signin)
                    .service(auth::signup)
                    .service(auth::activate_with_token)
                    .service(auth::activate_with_otp),
            )
            .service(web::scope("/services").service(services::get))
    });
}

pub fn setup_data(cfg: &mut web::ServiceConfig, settings: Arc<Settings>) {
    let conn = PostgresConnection::new(&settings.database.url())
        .expect("could not create a database connection");

    // create services
    let email_svc = Arc::new(EmailService::new(&settings.email).unwrap());
    let users_svc = Arc::new(UsersService::new(conn.clone()));
    let confirmations_svc =
        Arc::new(ConfirmationsService::new(conn.clone(), users_svc.clone()));
    let auth_svc = Arc::new(AuthSerivce::new(
        users_svc.clone(),
        confirmations_svc.clone(),
        email_svc.clone(),
    ));
    let services_svc = Arc::new(ServicesService::new(conn.clone()));

    // export data
    cfg.data(settings.clone())
        .data(email_svc)
        .data(users_svc)
        .data(confirmations_svc)
        .data(auth_svc)
        .data(services_svc);
}

pub fn setup_logging() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}
