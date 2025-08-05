use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    time::Instant,
};
use uuid::Uuid;

pub struct RequestLogger;

impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggerMiddleware { service }))
    }
}

pub struct RequestLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = Instant::now();
        let method = req.method().to_string();
        let path = req.path().to_string();
        let query_string = req.query_string().to_string();
        let request_id = Uuid::new_v4().to_string();
        let client_ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();

        // Log incoming request
        log::info!(
            "Request started - ID: {} | {} {} | IP: {} | Query: {}",
            request_id,
            method,
            path,
            client_ip,
            if query_string.is_empty() {
                "none"
            } else {
                &query_string
            }
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let result = fut.await;
            let duration = start_time.elapsed();

            match &result {
                Ok(response) => {
                    log::info!(
                        "Request completed - ID: {} | Status: {} | Duration: {:?}",
                        request_id,
                        response.status(),
                        duration
                    );
                }
                Err(error) => {
                    log::error!(
                        "Request failed - ID: {} | Error: {:?} | Duration: {:?}",
                        request_id,
                        error,
                        duration
                    );
                }
            }

            result
        })
    }
}
