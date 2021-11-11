extern crate actix_web;
extern crate derive_more;
extern crate dotenv;
extern crate serde;

mod handlers;
mod result;
mod services;
mod setup;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let listen_ep = env::var("ListenEndpoint").expect("listen address");

    HttpServer::new(|| App::new().configure(setup::setup_webserver))
        .bind(listen_ep)?
        .run()
        .await
}
