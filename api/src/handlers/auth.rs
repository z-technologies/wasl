use crate::result::ApiResult;
use crate::services::auth::AuthSerivce;

use data::context::DbContext;
use data::models::user::NewUser;

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

    Ok(HttpResponse::Ok().body("success"))
}

#[post("/signup")]
pub async fn signup(
    _ctx: web::Data<DbContext>,
    form: web::Json<NewUser>,
) -> ApiResult<HttpResponse> {
    form.validate()?;

    Ok(HttpResponse::Ok().body("success"))
}

#[derive(Deserialize)]
pub struct SigninForm {
    username: String,
    password: String,
}
