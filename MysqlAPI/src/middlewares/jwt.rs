use crate::models::claims_model::Claims;
use actix_service::{Service, Transform};
use actix_web::body::BoxBody; use actix_web::HttpMessage;
// 仍然需要这个来转换响应体类型
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::header::{self, HeaderName, HeaderValue},
    Error, HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use std::env;
use std::task::{Context, Poll};
use regex::Regex;
use std::collections::HashMap;
pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareMiddleware { service })
    }
}

pub struct JwtMiddlewareMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 获取请求的路径
        let path = req.path();
         // 接口请求以 /api/ 开头
        let re = Regex::new(r"^/api/.*").unwrap();
        if !re.is_match(path) {
            let fut = self.service.call(req);
            return Box::pin(async move { fut.await });
        }
        // 如果路径是 "/token" 或其他需要跳过认证的路径，直接返回
        if path.ends_with("/token") || path.contains("/tokenget") {
            // 直接调用下游服务，跳过 JWT 验证
            let fut = self.service.call(req);
            return Box::pin(async move { fut.await });
        }
       





        let auth_header = req.headers().get(header::AUTHORIZATION);

        if let Some(auth_header) = auth_header {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");

                    // 解码 JWT，获取 Claims
                    if let Ok(decoded_token) = decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(secret.as_ref()),
                        &Validation::default(),
                    ) {
                        // 获取 user_id
                        let user_id = decoded_token.claims.user_id;
                        // Token 有效，继续处理请求
                        let fut = self.service.call(req);
                        return Box::pin(async move { fut.await });
                    }
                }
            }
        }

        // 如果 JWT 校验失败，则在扩展中插入认证失败的标志符
       // 如果认证失败，设置标志符到请求的扩展中
       let mut flags = HashMap::new();
       flags.insert("auth_failed", true);
       flags.insert("is_admin", false);  // 你可以插入多个标志符
       req.extensions_mut().insert(flags);  // 将 HashMap 插入到扩展字段中
        let fut = self.service.call(req);
        return Box::pin(async move { fut.await });
    }
}
