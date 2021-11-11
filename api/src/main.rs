mod config;
mod handlers;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listen_endpoint = format!("{}:{}", config::LISTEN_ADDRESS, config::LISTEN_PORT);

    HttpServer::new(|| App::new().configure(config::configure_server))
        .bind(listen_endpoint)?
        .run()
        .await
}
