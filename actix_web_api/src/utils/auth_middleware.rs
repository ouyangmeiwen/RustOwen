// src/utils/auth_middleware.rs

use actix_web::{Error, HttpRequest};
use actix_service::Service;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::web::{Data, Json};
use futures::future::{ok, Ready};
use crate::config::Config;

pub fn auth_middleware<S>(
    req: &ServiceRequest,
    srv: &S,
) -> Ready<Result<ServiceResponse, Error>>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
{
    if let Some(auth_header) = req.headers().get("Authorization") {
        let token = auth_header.to_str().unwrap();
        if validate_token(token) {
            return srv.call(req);
        }
    }
    ok(req.error_response(HttpResponse::Unauthorized().body("Invalid token")))
}

fn validate_token(token: &str) -> bool {
    // 实现实际的 token 校验逻辑
    token == "valid-token" // 例子：只允许 "valid-token"
}
