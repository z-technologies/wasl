extern crate actix_web;
extern crate chrono;
extern crate config;
extern crate derive_more;
extern crate env_logger;
extern crate jsonwebtoken;
extern crate serde;
extern crate validator;

mod auth;
mod handlers;
mod result;
mod settings;
mod setup;

use crate::settings::Settings;

use business::services::{AuthSerivce, EmailService};
use data::context::{create_connection_pool, DbContext};

use actix_web::{App, HttpServer};
use std::sync::Arc;

const MAX_POOL_CONNECTIONS: u32 = 4;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup::setup_logging();

    let settings: Arc<Settings> = Arc::new(Settings::new().unwrap());

    let ctx = create_db_ctx(&settings);
    let listen_ep = settings.server.endpoint();

    HttpServer::new(move || {
        App::new()
            .data(settings.clone())
            .data(ctx.clone())
            .data(AuthSerivce::new(ctx.clone()))
            .data(EmailService::new(
                &settings.email.smtp_host,
                settings.email.smtp_port,
                &settings.email.smtp_username,
                &settings.email.smtp_password,
                settings.email.require_tls,
            ))
            .configure(setup::setup_handlers)
    })
    .bind(listen_ep)?
    .run()
    .await
}

fn create_db_ctx(settings: &Settings) -> DbContext {
    let db_url = settings.database.url();
    let db_pool = create_connection_pool(&db_url, MAX_POOL_CONNECTIONS)
        .expect("could not create a database pool");

    DbContext::new(db_pool)
}
