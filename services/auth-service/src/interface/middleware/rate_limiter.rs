use actix_web::{
    body::BoxBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::{
    collections::HashMap,
    future::{ready, Ready},
    rc::Rc,
    sync::Mutex,
    time::{Duration, Instant},
};

pub struct RateLimiter {
    max_requests: u32,
    window_duration: Duration,
    storage: Rc<Mutex<HashMap<String, (u32, Instant)>>>,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_duration: Duration) -> Self {
        Self {
            max_requests,
            window_duration,
            storage: Rc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static + actix_web::body::MessageBody,
{
    type Response = ServiceResponse<BoxBody>; // <--- Key Change
    type Error = Error;
    type Transform = RateLimiterMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware {
            service,
            max_requests: self.max_requests,
            window_duration: self.window_duration,
            storage: self.storage.clone(),
        }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    max_requests: u32,
    window_duration: Duration,
    storage: Rc<Mutex<HashMap<String, (u32, Instant)>>>,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static + actix_web::body::MessageBody,
{
    type Response = ServiceResponse<BoxBody>; // <--- Key Change
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let client_ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();

        let mut storage = self.storage.lock().unwrap();
        let now = Instant::now();

        // Clean up expired entries
        storage.retain(|_, (_, timestamp)| now.duration_since(*timestamp) < self.window_duration);

        let (count, first_request) = storage.entry(client_ip.clone()).or_insert((0, now));

        if now.duration_since(*first_request) >= self.window_duration {
            *count = 1;
            *first_request = now;
        } else {
            *count += 1;
        }

        if *count > self.max_requests {
            drop(storage);
            Box::pin(async move {
                let response = HttpResponse::TooManyRequests()
                    .json(serde_json::json!({ "error": "Rate limit exceeded" }))
                    .map_into_boxed_body();

                Ok(ServiceResponse::new(req.into_parts().0, response))
            })
        } else {
            drop(storage);
            let fut = self.service.call(req);
            Box::pin(async move { fut.await.map(|res| res.map_into_boxed_body()) })
        }
    }
}
