use crate::result::Result;

use business::services::ServicesService;
use data::models::KeyType;

use actix_web::{get, web, HttpResponse};

#[get("/{id}")]
pub async fn get(
    id: web::Path<KeyType>,
    services_svc: web::Data<ServicesService>,
) -> Result<HttpResponse> {
    let service =
        web::block(move || services_svc.get_ref().get_service_by_id(id.0))
            .await?;

    Ok(HttpResponse::Ok().json(service))
}
