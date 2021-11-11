use actix_web::{get, web, Responder};

#[get("/echo/{data}")]
pub async fn echo(web::Path(data): web::Path<String>) -> impl Responder {
    data
}
