use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::configs::ratelimitconfig::GLOBAL_PATH_LIMITS;

#[derive(Clone)] // Derive Clone for RateLimitMiddleware
pub struct LimitMiddleware {
    path_hits: Arc<Mutex<HashMap<String, (u64, Instant)>>>, // Wrap the Mutex in an Arc
    limit_per_second_default: u64,
    time_window_secs_default: u64, // Added field for the time window
}

impl LimitMiddleware {
    // Updated constructor to take time window as a parameter
    pub fn new(limit_per_second_default: u64, time_window_secs_default: u64) -> Self {
        LimitMiddleware {
            path_hits: Arc::new(Mutex::new(HashMap::new())),
            limit_per_second_default,
            time_window_secs_default, // Initialize the time window
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for LimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LimitMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LimitMiddlewareService {
            service,
            path_hits: Arc::clone(&self.path_hits), // Use Arc::clone to pass the reference
            limit_per_second_default: self.limit_per_second_default,
            time_window_secs_default: self.time_window_secs_default, // Pass time window to service
        })
    }
}

pub struct LimitMiddlewareService<S> {
    service: S,
    path_hits: Arc<Mutex<HashMap<String, (u64, Instant)>>>,
    limit_per_second_default: u64,
    time_window_secs_default: u64, // Store the time window in the service
}

impl<S, B> Service<ServiceRequest> for LimitMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();
        let mut path_hits = self.path_hits.lock().unwrap();
        let now = Instant::now();
        let (count, last_access_time) = path_hits.entry(path.clone()).or_insert((0, now));

        // let global_limits = GLOBAL_PATH_LIMITS.read().unwrap();
        // // 打印 GLOBAL_PATH_LIMITS 的所有内容
        // println!("GLOBAL_PATH_LIMITS: {:?}", *global_limits);

        // Get the per-path rate limit configuration or fallback to the default
        let (limit_per_second, time_window_secs) = GLOBAL_PATH_LIMITS
            .read()
            .unwrap()
            .get(&path)
            .cloned()
            .unwrap_or((self.limit_per_second_default, self.time_window_secs_default));

        // Use the configured time window for rate limiting
        if last_access_time.elapsed().as_secs() < time_window_secs {
            *count += 1;
        } else {
            *count = 1;
        }
        // 打印当前路径的速率限制配置
        println!(
            "Path: {},current:{}, Limit: {}/{}s",
            path, *count, limit_per_second, time_window_secs
        );
        *last_access_time = now;

        if *count > limit_per_second {
            req.extensions_mut().insert(HashMap::<&str, String>::from([(
                "rate_limit_exceeded",
                "true".to_string(),
            )]));
        }

        let fut = self.service.call(req);
        Box::pin(async move { fut.await })
    }
}
