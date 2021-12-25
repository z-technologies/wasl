extern crate actix_web;
extern crate chrono;
extern crate config;
extern crate derive_more;
extern crate env_logger;
extern crate jsonwebtoken;
extern crate pem;
extern crate serde;
extern crate validator;

mod auth;
mod handlers;
mod middlewares;
mod result;
mod settings;
mod setup;

use crate::middlewares::auth::AuthMiddlewareFactory;
use crate::settings::Settings;

use actix_web::{middleware::Logger, App, HttpServer};

use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup::setup_logging();

    let settings = Arc::new(Settings::new().unwrap());
    let listen_ep = settings.server.endpoint();

    HttpServer::new(move || {
        App::new()
            .wrap(AuthMiddlewareFactory::new(settings.clone()))
            .wrap(Logger::default())
            .configure(setup::setup_handlers)
            .configure(|cfg| setup::setup_data(cfg, settings.clone()))
    })
    .bind(listen_ep)?
    .run()
    .await
}
