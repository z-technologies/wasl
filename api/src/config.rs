use actix_web::web;

pub fn configure_server(cfg: &mut web::ServiceConfig) {
    cfg.service({
        web::scope("/api").service(web::scope("/v1").service(crate::handlers::echo::echo))
    });
}
