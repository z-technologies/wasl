use crate::handlers::auth;
use crate::handlers::echo;
use crate::settings::Settings;
use crate::Arc;
use business::services::AuthSerivce;
use business::services::EmailService;
use data::context::create_connection_pool;
use data::context::DbContext;

use actix_web::web;

use std::env;

const MAX_POOL_CONNECTIONS: u32 = 4;

pub fn setup_handlers(cfg: &mut web::ServiceConfig) {
    cfg.service({
        web::scope("/api/v1")
            .service(web::scope("/test").service(echo::echo))
            .service(
                web::scope("/auth")
                    .service(auth::signin)
                    .service(auth::signup)
                    .service(auth::set_initial_password),
            )
    });
}

pub fn setup_data(cfg: &mut web::ServiceConfig, settings: Arc<Settings>) {
    // create database context
    let db_url = settings.database.url();
    let db_pool = create_connection_pool(&db_url, MAX_POOL_CONNECTIONS)
        .expect("could not create a database pool");
    let ctx = DbContext::new(db_pool);

    // create services
    let email_svc = Arc::new(EmailService::new(&settings.email).unwrap());
    let auth_svc = Arc::new(AuthSerivce::new(ctx.clone(), email_svc.clone()));

    // export data
    cfg.data(settings.clone())
        .data(ctx)
        .data(auth_svc)
        .data(email_svc);
}

pub fn setup_logging() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}
