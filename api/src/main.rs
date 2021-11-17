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
use business::services::AuthSerivce;

use data::context::{create_connection_pool, DbContext};

use actix_web::{App, HttpServer};
use std::sync::Arc;

const MAX_POOL_CONNECTIONS: u32 = 4;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup::setup_logging();

    let settings: Arc<Settings> = Arc::new(Settings::new().unwrap());

    let listen_ep = settings.server.endpoint();
    let db_url = settings.database.url();
    let db_pool = create_connection_pool(&db_url, MAX_POOL_CONNECTIONS)
        .expect("could not create a database pool");

    let ctx = DbContext::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .data(settings.clone())
            .data(ctx.clone())
            .data(AuthSerivce { ctx: ctx.clone() })
            .configure(setup::setup_handlers)
    })
    .bind(listen_ep)?
    .run()
    .await
}
