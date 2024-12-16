use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::{LocalBoxFuture, Ready};
use std::collections::HashMap;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::runtime::Handle;
use tokio::sync::Semaphore;
pub struct RateLimitMiddleware {
    semaphore: Arc<Semaphore>,
}

impl RateLimitMiddleware {
    pub fn new(max_requests: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_requests)),
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
    type InitError = ();
    type Transform = RateLimitMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures_util::future::ok(RateLimitMiddlewareService {
            service,
            semaphore: self.semaphore.clone(),
        })
    }
}

pub struct RateLimitMiddlewareService<S> {
    service: S,
    semaphore: Arc<Semaphore>,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddlewareService<S>
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
        let mut flags: HashMap<&str, String> = HashMap::new();
        let permit = self.semaphore.clone();
        // 获取当前 Tokio 运行时的 handle
        let handle = Handle::current();
        // 使用 block_on 阻塞当前线程，等待 acquire 完成
        let _permit = handle.block_on(permit.acquire());
        match _permit {
            Ok(_) => {
                if _permit.is_err() {
                    // 如果无法获取 permit，设置限流标志
                    flags.insert("rate_limited", "true".to_string());
                }
            }
            Err(e) => {}
        }
        req.extensions_mut().insert(flags); // 修改 `req`，不消费它
        let fut: <S as Service<ServiceRequest>>::Future = self.service.call(req); // 这里也没有移除 `req`
        Box::pin(async move { fut.await })
    }
}
