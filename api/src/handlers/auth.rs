use crate::result::{ApiError, ApiResult};
use crate::services::auth::AuthSerivce;

use data::context::DbContext;

use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

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
pub async fn signup() -> impl Responder {
    format!("Hello from sign up")
}

#[derive(Deserialize)]
pub struct SigninForm {
    username: String,
    password: String,
}
