use crate::auth::token::Claims;
use crate::result::{ApiError, Result};
use crate::settings::Settings;

use business::security::password::make_hash;
use business::services::auth::AuthSerivce;
use data::models::user::NewUser;
use data::models::validate::RE_USERNAME;

use actix_web::{get, post, web, HttpResponse};
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
        user,
        groups,
        settings.security.token_expiration_seconds,
    )?
    .encode(&settings.security.private_key()?)?;

    Ok(HttpResponse::Ok().body(token))
}

#[post("/signup")]
pub async fn signup(
    form: web::Json<SignupForm>,
    auth: web::Data<Arc<AuthSerivce>>,
) -> Result<HttpResponse> {
    form.validate()?;

    let new_user = form.into_inner().into_new_user()?;
    let user = web::block(move || auth.signup(&new_user)).await?;

    Ok(HttpResponse::Created().json(user))
}

#[get("/token-activate/{username}/{token}")]
pub async fn activate_with_token(
    auth: web::Data<Arc<AuthSerivce>>,
    web::Path((username, token)): web::Path<(String, String)>,
) -> Result<HttpResponse> {
    web::block(move || auth.activate_with_token(&username, &token)).await?;
    Ok(HttpResponse::Accepted().finish())
}

#[get("/otp-activate/{username}/{token}")]
pub async fn activate_with_otp(
    auth: web::Data<Arc<AuthSerivce>>,
    web::Path((username, token)): web::Path<(String, String)>,
) -> Result<HttpResponse> {
    web::block(move || auth.activate_with_otp(&username, &token)).await?;
    Ok(HttpResponse::Accepted().finish())
}

#[derive(Deserialize, Validate)]
pub struct SigninForm {
    #[validate(regex = "RE_USERNAME")]
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SignupForm {
    #[validate(regex = "RE_USERNAME")]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}

impl SignupForm {
    fn into_new_user(self) -> Result<NewUser> {
        let password = match make_hash(&self.password) {
            Ok(password) => password,
            Err(err) => return Err(ApiError::UserError(err)),
        };

        Ok(NewUser {
            username: self.username,
            email: self.email,
            password_hash: password,
        })
    }
}
