use crate::auth::token::Claims;
use crate::settings::Settings;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};

use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

const AUTHORIZATION_HEADER: &str = "Authorization";

pub struct AuthMiddleware<S> {
    service: S,
    settings: Arc<Settings>,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if let Some(auth_header_value) = req.headers().get(AUTHORIZATION_HEADER)
        {
            if let Ok(auth_header) = auth_header_value.to_str() {
                if let Ok(pem_pk) = self.settings.security.private_key_pem() {
                    if let Ok(claims) =
                        Claims::from_bearer(auth_header, &pem_pk)
                    {
                        req.extensions_mut().insert::<Claims>(claims);
                    }
                }
            }
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

pub struct AuthMiddlewareFactory {
    settings: Arc<Settings>,
}

impl<S, B> Transform<S> for AuthMiddlewareFactory
where
    S: Service<
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service,
            settings: self.settings.clone(),
        }))
    }
}

impl AuthMiddlewareFactory {
    pub fn new(settings: Arc<Settings>) -> Self {
        AuthMiddlewareFactory { settings }
    }
}
