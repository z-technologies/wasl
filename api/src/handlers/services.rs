use crate::auth::groups::AuthGroup;
use crate::auth::identity::Identity;
use crate::result::Result;
use crate::settings::Settings;

use wasl::data::models::{KeyType, NewService, User};
use wasl::services::ServicesService;
use wasl::services::UsersService;

use actix_web::{delete, get, post, web, HttpResponse};
use bigdecimal::BigDecimal;
use serde::Deserialize;

use std::sync::Arc;

#[get("/{id}")]
pub async fn get(
    id: web::Path<KeyType>,
    services_svc: web::Data<ServicesService>,
) -> Result<HttpResponse> {
    let service = web::block(move || services_svc.get_ref().get(id.0)).await?;

    Ok(HttpResponse::Ok().json(service))
}

#[post("/")]
pub async fn add(
    auth: Identity,
    users_svc: web::Data<Arc<UsersService>>,
    services_svc: web::Data<Arc<ServicesService>>,
    service: web::Json<AddSerivceFrom>,
) -> Result<HttpResponse> {
    auth.has(AuthGroup::Provider)?;

    let user = auth.user(users_svc.get_ref().clone()).await?;
    let svc = web::block(move || {
        services_svc
            .get_ref()
            .create(&service.0.new_service_for(&user))
    })
    .await?;

    Ok(HttpResponse::Ok().json(svc))
}

#[delete("/{id}")]
pub async fn delete(
    auth: Identity,
    id: web::Path<KeyType>,
    users_svc: web::Data<Arc<UsersService>>,
    services_svc: web::Data<ServicesService>,
) -> Result<HttpResponse> {
    auth.has(AuthGroup::Provider)?;

    let user = auth.user(users_svc.get_ref().clone()).await?;

    web::block(move || {
        let service = services_svc.get_ref().get(id.0)?;
        services_svc.get_ref().delete(service, &user)
    })
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/{id}/reserve")]
pub async fn reserve(
    auth: Identity,
    id: web::Path<KeyType>,
    users_svc: web::Data<Arc<UsersService>>,
    services_svc: web::Data<Arc<ServicesService>>,
    period: web::Json<DateTimePeriod>,
    settings: web::Data<Arc<Settings>>,
) -> Result<HttpResponse> {
    auth.has(AuthGroup::Customer)?;

    let user = auth.user(users_svc.get_ref().clone()).await?;
    let private_key = settings.security.private_key()?;

    let reservation = web::block(move || {
        let service = services_svc.get_ref().get(id.0)?;

        services_svc.make_reservation(
            &service,
            &user,
            period.begin,
            period.end,
            &private_key,
        )
    })
    .await?;

    Ok(HttpResponse::Ok().json(reservation))
}

#[derive(Deserialize)]
pub struct AddSerivceFrom {
    pub title: String,
    pub description: String,
    pub price: BigDecimal,
    pub available_begin: Option<chrono::NaiveTime>,
    pub available_end: Option<chrono::NaiveTime>,
}

#[derive(Deserialize)]
pub struct DateTimePeriod {
    pub begin: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

impl AddSerivceFrom {
    fn new_service_for(self, provider: &User) -> NewService {
        NewService {
            title: self.title,
            description: self.description,
            price: self.price,
            available_begin: self.available_begin,
            available_end: self.available_end,
            user_id: provider.id,
        }
    }
}
