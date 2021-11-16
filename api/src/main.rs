extern crate actix_web;
extern crate chrono;
extern crate derive_more;
extern crate dotenv;
extern crate env_logger;
extern crate jsonwebtoken;
extern crate serde;
extern crate validator;

mod auth;
mod handlers;
mod result;
mod setup;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let listen_ep = env::var("LISTEN_ENDPOINT").expect("listen address");

    HttpServer::new(|| App::new().configure(setup::setup_webserver))
        .bind(listen_ep)?
        .run()
        .await
}
