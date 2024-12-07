use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::header,
    Error, HttpResponse,
};
use actix_web::body::BoxBody; // 仍然需要这个来转换响应体类型
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use std::task::{Context, Poll};
use std::env;

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID or other identifier
    pub exp: usize,  // Expiration time
}

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
        // 如果路径是 "/generate_token" 或其他需要跳过认证的路径，直接返回
        if path.ends_with("/token") {
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
                        let user_id = decoded_token.claims.sub;
                        // Token 有效，继续处理请求
                        let fut = self.service.call(req);
                        return Box::pin(async move { fut.await });
                    }
                }
            }
        }
        // 如果 token 无效或缺失，返回 Unauthorized
        let res: HttpResponse = HttpResponse::Unauthorized().finish();
        let fut = self.service.call(req);
        return Box::pin(async move { fut.await });
    }
}
