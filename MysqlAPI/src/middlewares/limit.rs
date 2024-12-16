use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Clone)] // Derive Clone for RateLimitMiddleware
pub struct RateLimitMiddleware {
    path_hits: Arc<Mutex<HashMap<String, (u64, Instant)>>>, // Wrap the Mutex in an Arc
    limit_per_second: u64,
}

impl RateLimitMiddleware {
    pub fn new(limit_per_second: u64) -> Self {
        RateLimitMiddleware {
            path_hits: Arc::new(Mutex::new(HashMap::new())), // Initialize with Arc<Mutex>
            limit_per_second,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RateLimitMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RateLimitMiddlewareService {
            service,
            path_hits: Arc::clone(&self.path_hits), // Use Arc::clone to pass the reference
            limit_per_second: self.limit_per_second,
        })
    }
}

pub struct RateLimitMiddlewareService<S> {
    service: S,
    path_hits: Arc<Mutex<HashMap<String, (u64, Instant)>>>, // Arc<Mutex> shared across threads
    limit_per_second: u64,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddlewareService<S>
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

        if last_access_time.elapsed().as_secs() < 10 {
            *count += 1;
        } else {
            *count = 1;
        }
        println!("count:{}", *count);

        *last_access_time = now;

        if *count > self.limit_per_second {
            req.extensions_mut().insert(HashMap::<&str, String>::from([(
                "rate_limit_exceeded",
                "true".to_string(),
            )]));
        }

        let fut = self.service.call(req);
        Box::pin(async move { fut.await })
    }
}
