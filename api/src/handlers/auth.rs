use crate::result::ApiResult;

use business::services::auth::AuthSerivce;
use data::models::user::NewUser;

use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use validator::Validate;

#[post("/signin")]
pub async fn signin(
    auth: web::Data<AuthSerivce>,
    form: web::Json<SigninForm>,
) -> ApiResult<HttpResponse> {
    let user =
        web::block(move || auth.signin(&form.username, &form.password)).await?;

    // TODO:
    // handle token creation

    Ok(HttpResponse::Ok().json(user))
}

#[post("/signup")]
pub async fn signup(
    auth: web::Data<AuthSerivce>,
    form: web::Json<NewUser>,
) -> ApiResult<HttpResponse> {
    form.validate()?;

    let user = web::block(move || auth.signup(&form)).await?;
    Ok(HttpResponse::Created().json(user))
}

#[derive(Deserialize)]
pub struct SigninForm {
    username: String,
    password: String,
}
