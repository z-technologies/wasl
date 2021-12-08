use crate::handlers::{auth, echo, services};
use crate::settings::Settings;

use business::services::*;
use data::context::{create_connection_pool, DbContext};

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
    // create database context
    let pool = create_connection_pool(
        &settings.database.url(),
        settings.database.max_pool_connections,
    )
    .expect("could not create a database pool");
    let ctx = DbContext::new(pool);

    // create services
    let email_svc = Arc::new(EmailService::new(&settings.email).unwrap());
    let auth_svc = Arc::new(AuthSerivce::new(ctx.clone(), email_svc.clone()));
    let services_svc = Arc::new(ServicesService::new(ctx.clone()));

    // export data
    cfg.data(settings.clone())
        .data(ctx)
        .data(auth_svc)
        .data(email_svc)
        .data(services_svc);
}

pub fn setup_logging() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}
