use crate::result::Result;

use actix_web::HttpResponse;
use actix_web::{get, web};

#[get("/echo/{data}")]
pub async fn echo(web::Path(data): web::Path<String>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body(data))
}
