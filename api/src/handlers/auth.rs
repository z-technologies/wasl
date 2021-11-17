use crate::auth::token::Claims;
use crate::result::Result;

use business::services::auth::AuthSerivce;
use data::context::DbContext;
use data::models::user::NewUser;
use data::models::validate::RE_USERNAME;

use actix_web::{post, put, web, HttpResponse};
use serde::Deserialize;
use validator::Validate;

#[post("/signin")]
pub async fn signin(
    auth: web::Data<AuthSerivce>,
    form: web::Json<SigninForm>,
) -> Result<HttpResponse> {
    form.validate()?;

    let (user, groups) =
        web::block(move || auth.signin(&form.username, &form.password)).await?;

    let token = Claims::for_user(&user, groups)?.encode()?;
    Ok(HttpResponse::Ok().body(token))
}

#[post("/signup")]
pub async fn signup(
    auth: web::Data<AuthSerivce>,
    form: web::Json<NewUser>,
) -> Result<HttpResponse> {
    form.validate()?;

    let user = web::block(move || auth.signup(&form)).await?;

    // TODO:
    // handle email verification

    Ok(HttpResponse::Created().json(user))
}

#[put("/set-password")]
pub async fn set_password(
    auth: web::Data<AuthSerivce>,
    form: web::Json<UpdatePasswordForm>,
) -> Result<HttpResponse> {
    form.validate()?;

    web::block(move || {
        auth.set_password(&form.username, &form.password, &form.token)
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
