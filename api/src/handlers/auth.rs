use crate::auth::token::Claims;
use crate::result::Result;

use business::services::auth::AuthSerivce;
use data::models::user::NewUser;

use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use validator::Validate;

#[post("/signin")]
pub async fn signin(
    auth: web::Data<AuthSerivce>,
    form: web::Json<SigninForm>,
) -> Result<HttpResponse> {
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

#[derive(Deserialize)]
pub struct SigninForm {
    username: String,
    password: String,
}
