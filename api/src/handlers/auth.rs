use crate::result::ApiResult;

use business::services::auth::AuthSerivce;
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
    let _user = web::block(move || {
        let auth = AuthSerivce { ctx: ctx.get_ref() };
        auth.signin(&form.username, &form.password)
    })
    .await?;

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

    let model = web::block(move || {
        let auth = AuthSerivce { ctx: ctx.get_ref() };
        auth.signup(&form)
    })
    .await?;

    Ok(HttpResponse::Created().json(model))
}

#[derive(Deserialize)]
pub struct SigninForm {
    username: String,
    password: String,
}
