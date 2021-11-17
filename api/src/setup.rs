use crate::handlers::auth;
use crate::handlers::echo;

use business::services::auth::AuthSerivce;

use actix_web::web;
use std::env;

const MAX_POOL_CONNECTIONS: u32 = 4;

pub fn setup_webserver(cfg: &mut web::ServiceConfig) {
    let db_url = env::var("DB_URL").expect("database url");
    let db_pool =
        data::context::create_connection_pool(&db_url, MAX_POOL_CONNECTIONS)
            .expect("could not create a database pool");

    let ctx = data::context::DbContext::new(db_pool);

    cfg.service({
        web::scope("/api/v1")
            .service(web::scope("/test").service(echo::echo))
            .service(
                web::scope("/auth")
                    .service(auth::signin)
                    .service(auth::signup)
                    .service(auth::set_password),
            )
    })
    .data(ctx.clone())
    .data(AuthSerivce { ctx });
}
