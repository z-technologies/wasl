use crate::auth::groups::AuthGroup;
use crate::auth::identity::Identity;
use crate::result::Result;

use wasl::data::models::{KeyType, NewService, User};
use wasl::services::ServicesService;
use wasl::services::UsersService;

use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;

use std::sync::Arc;

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

#[derive(Deserialize)]
pub struct AddSerivceFrom {
    pub title: String,
    pub description: String,
    pub available_begin: Option<chrono::NaiveTime>,
    pub available_end: Option<chrono::NaiveTime>,
}

impl AddSerivceFrom {
    fn new_service_for(self, user: &User) -> NewService {
        NewService {
            title: self.title,
            description: self.description,
            available_begin: self.available_begin,
            available_end: self.available_end,
            user_id: user.id,
        }
    }
}
