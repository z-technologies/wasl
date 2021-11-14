use crate::result::{ApiError, ApiResult};
use crate::services::auth::AuthSerivce;

use data::context::DbContext;
use data::models::user::NewUser;
use data::repos::Repo;

use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use validator::Validate;

#[post("/signin")]
pub async fn signin(
    ctx: web::Data<DbContext>,
    form: web::Json<SigninForm>,
) -> ApiResult<HttpResponse> {
    let auth = AuthSerivce { ctx: ctx.get_ref() };
    let _user = auth.signin(&form.username, &form.password)?;

    // TODO:
    // handle token creation
    Ok(HttpResponse::Ok().body("success"))
}

#[post("/signup")]
pub async fn signup(
    ctx: web::Data<DbContext>,
    form: web::Json<NewUser>,
) -> ApiResult<HttpResponse> {
    form.validate()?;

    if ctx.get_ref().users().duplicate_username(&form.username)? {
        return Err(ApiError::UsernameAlreadyInUse);
    }

    if ctx.get_ref().users().duplicate_email(&form.email)? {
        return Err(ApiError::EmailAlreadyInUse);
    }

    // TODO:
    // handle email verification

    let model = web::block(move || ctx.get_ref().users().insert(&form)).await?;
    Ok(HttpResponse::Created().json(model))
}

#[derive(Deserialize)]
pub struct SigninForm {
    username: String,
    password: String,
}
