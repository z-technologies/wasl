use crate::auth::token::Claims;
use crate::result::Result;
use crate::settings::Settings;

use business::services::auth::AuthSerivce;
use data::models::user::NewUser;
use data::models::validate::RE_USERNAME;

use actix_web::{post, put, web, HttpResponse};
use serde::Deserialize;
use validator::Validate;

use std::sync::Arc;

#[post("/signin")]
pub async fn signin(
    form: web::Json<SigninForm>,
    auth: web::Data<Arc<AuthSerivce>>,
    settings: web::Data<Arc<Settings>>,
) -> Result<HttpResponse> {
    form.validate()?;

    let (user, groups) =
        web::block(move || auth.signin(&form.username, &form.password)).await?;

    let token = Claims::for_user(
        &user,
        groups,
        settings.security.token_expiration_seconds,
    )?
    .encode(&settings.security.private_key()?)?;

    Ok(HttpResponse::Ok().body(token))
}

#[post("/signup")]
pub async fn signup(
    form: web::Json<NewUser>,
    auth: web::Data<Arc<AuthSerivce>>,
) -> Result<HttpResponse> {
    form.validate()?;

    let user = web::block(move || auth.signup(&form)).await?;
    Ok(HttpResponse::Created().json(user))
}

#[put("/set-initial-password")]
pub async fn set_initial_password(
    auth: web::Data<AuthSerivce>,
    form: web::Json<UpdatePasswordForm>,
) -> Result<HttpResponse> {
    form.validate()?;

    web::block(move || {
        auth.set_initial_password(&form.username, &form.password, &form.token)
    })
    .await?;

    Ok(HttpResponse::Accepted().finish())
}

#[derive(Deserialize, Validate)]
pub struct SigninForm {
    #[validate(regex = "RE_USERNAME")]
    username: String,
    password: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdatePasswordForm {
    #[validate(regex = "RE_USERNAME")]
    username: String,
    #[validate(length(min = 6, max = 32))]
    password: String,
    token: String,
}
