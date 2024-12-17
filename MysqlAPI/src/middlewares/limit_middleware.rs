use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use actix_web::body::EitherBody;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpResponse};
use async_std::sync::Mutex;
use futures::Future;

use crate::configs::envconfig::STATIC_CONFIG;
use crate::configs::ratelimitconfig::GLOBAL_PATH_LIMITS;
use crate::models::apiresponse_model::ApiResponse;
use crate::models::claims_model::Claims;
use crate::models::config_model::Config;
use actix_web::http::header::{self, HeaderName, HeaderValue};
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::task::{Context, Poll};
use std::time::Instant; // 使用 async-std 的异步 Mutex
#[derive(Clone)] // Derive Clone for RateLimitMiddleware
pub struct RateLimitMiddleware {
    path_hits: Arc<Mutex<HashMap<String, (u64, Instant)>>>, // Wrap the Mutex in an Arc
    limit_per_second_default: u64,
    time_window_secs_default: u64, // Added field for the time window
}

impl RateLimitMiddleware {
    // Updated constructor to take time window as a parameter
    pub fn new(limit_per_second_default: u64, time_window_secs_default: u64) -> Self {
        RateLimitMiddleware {
            path_hits: Arc::new(Mutex::new(HashMap::new())),
            limit_per_second_default,
            time_window_secs_default, // Initialize the time window
        }
    }
}

impl<S: 'static, B> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = RateLimitMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RateLimitMiddlewareService {
            service: Rc::new(RefCell::new(service)),
            path_hits: Arc::clone(&self.path_hits), // Use Arc::clone to pass the reference
            limit_per_second_default: self.limit_per_second_default,
            time_window_secs_default: self.time_window_secs_default, // Pass time window to service
        })
    }
}

pub struct RateLimitMiddlewareService<S> {
    service: Rc<RefCell<S>>,
    path_hits: Arc<Mutex<HashMap<String, (u64, Instant)>>>,
    limit_per_second_default: u64,
    time_window_secs_default: u64, // Store the time window in the service
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddlewareService<S>
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
        let path_hits = Arc::clone(&self.path_hits);

        let path = req.path().to_string();
        // 获取路径的速率限制配置，若无配置则使用默认值
        let (limit_per_second, time_window_secs) = GLOBAL_PATH_LIMITS
            .read()
            .unwrap()
            .get(&path)
            .cloned()
            .unwrap_or((self.limit_per_second_default, self.time_window_secs_default));
        // 异步地获取 Mutex 锁
        Box::pin(async move {
            let mut path_hits = path_hits.lock().await; // 使用 `.await` 获取锁
            let now = Instant::now();
            let (count, last_access_time) = path_hits.entry(path.clone()).or_insert((0, now));
            // 判断是否超出请求限制
            if last_access_time.elapsed().as_secs() < time_window_secs {
                *count += 1;
            } else {
                *count = 1;
            }
            *last_access_time = now;

            // 超出限制时返回 429 Too Many Requests
            if *count > limit_per_second {
                req.extensions_mut().insert(HashMap::<&str, String>::from([(
                    "rate_limit_exceeded",
                    "true".to_string(),
                )]));
                return Ok(req.into_response(
                    HttpResponse::TooManyRequests()
                        .json(ApiResponse::<()>::error("TooManyRequests"))
                        .map_into_right_body(),
                ));
            }
            // 调用下游服务
            svc.call(req).await.map(ServiceResponse::map_into_left_body)
        })
    }
}
