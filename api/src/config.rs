use actix_web::web;

pub const LISTEN_ADDRESS: &'static str = "127.0.0.1";
pub const LISTEN_PORT: u32 = 8080;

pub fn configure_server(cfg: &mut web::ServiceConfig) {
    cfg.service({
        web::scope("/api").service(web::scope("/v1").service(crate::routes::echo::echo))
    });
}
