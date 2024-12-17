use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use actix_web::body::EitherBody;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpResponse};
use futures::Future;

use crate::configs::envconfig::STATIC_CONFIG;
use crate::models::apiresponse_model::ApiResponse;
use crate::models::claims_model::Claims;
use crate::models::config_model::Config;
use actix_web::http::header::{self, HeaderName, HeaderValue};
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::task::{Context, Poll};

pub struct Auth {}

impl<S: 'static, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleWare<S>;
    type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures::future::ok(AuthMiddleWare {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct AuthMiddleWare<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleWare<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    /* fn poll_ready(
        &mut self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    } */

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        Box::pin(async move {
            // 获取请求的路径
            let path = req.path();
            // 接口请求以 /api/ 开头
            let re = Regex::new(r"^/api/.*").unwrap();
            if !re.is_match(path) {
                svc.call(req).await.map(ServiceResponse::map_into_left_body)
            }
            // 如果路径是 "/token" 或其他需要跳过认证的路径，直接返回
            else if path.ends_with("/token") || path.contains("/tokenget") {
                // 直接调用下游服务，跳过 JWT 验证
                svc.call(req).await.map(ServiceResponse::map_into_left_body)
            } else {
                let authorization = req
                    .headers()
                    .get("authorization")
                    .map(|v| v.to_str())
                    .unwrap_or(Ok(""))
                    .unwrap();
                let parts: Vec<&str> = authorization.split(' ').collect();
                if parts.len() != 2 || parts.is_empty() || parts[0].to_lowercase() != "bearer" {
                    return Ok(req.into_response(
                        HttpResponse::Unauthorized().finish().map_into_right_body(),
                    ));
                }
                let token = parts[1];
                println!("token:{}", token);
                let config: Config = STATIC_CONFIG.read().unwrap().clone(); //智能指针
                let secret = &config.secret_key;
                println!("secret:{}", secret);
                let mut flags: HashMap<&str, String> = HashMap::new();
                // 解码 JWT，获取 Claims
                if let Ok(decoded_token) = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret.as_ref()),
                    &Validation::default(),
                ) {
                    flags.insert("user_id", decoded_token.claims.user_id.to_string()); //remove
                    println!("user_id:{}", decoded_token.claims.user_id.to_string());
                    flags.insert("user_role", decoded_token.claims.role.to_string()); //remove
                    println!("user_role:{}", decoded_token.claims.user_id.to_string());
                    req.extensions_mut().insert(flags); // 将 HashMap 插入到扩展字段中
                    svc.call(req).await.map(ServiceResponse::map_into_left_body)
                } else {
                    flags.insert("auth_failed", "true".to_string());
                    req.extensions_mut().insert(flags); // 将 HashMap 插入到扩展字段中

                    //svc.call(req).await.map(ServiceResponse::map_into_left_body)
                    return Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(ApiResponse::<()>::error("Unauthorized"))
                            .map_into_right_body(),
                    ));
                }
            }
        })
    }
}
