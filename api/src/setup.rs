use crate::handlers::auth;
use crate::handlers::echo;

use actix_web::web;
use dotenv::dotenv;

use std::env;

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

pub fn setup_logging() {
    dotenv().ok();
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}
