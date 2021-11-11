extern crate dotenv;

mod handlers;
mod setup;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let listen_ep = env::var("ListenEndpoint").expect("listen address");
    println!("{}", listen_ep);

    HttpServer::new(|| App::new().configure(setup::setup_webserver))
        .bind(listen_ep)?
        .run()
        .await
}
