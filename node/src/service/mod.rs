mod error;
mod node_api;

use std::convert::Infallible;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;

use axum::{
    body::Body, extract::OriginalUri, handler::Handler, http::Request, http::StatusCode,
    response::Response, routing::get, Extension, Json, Router,
};
use error::ServiceError;
use futures::Future;
use nmos_rs_model::Model;
use serde_json::json;
use tower::Service;

use self::node_api::{
    get_device, get_devices, get_flow, get_flows, get_receiver, get_receivers, get_self,
    get_sender, get_senders, get_source, get_sources,
};

#[derive(Debug, Clone)]
pub struct NmosService {
    router: Router,
}

impl NmosService {
    pub fn new(model: Arc<Model>) -> Self {
        let router = Router::new()
            .route(
                "/",
                get(|| async { Json(json!(["x-manifest/", "x-nmos/"])) }),
            )
            .route("/x-nmos/", get(|| async { Json(json!(["node/"])) }))
            .route("/x-nmos/node/", get(|| async { Json(json!(["v1.0/"])) }))
            .route(
                "/x-nmos/node/v1.0/",
                get(|| async {
                    Json(json!([
                        "devices/",
                        "flows/",
                        "receivers/",
                        "self/",
                        "senders/",
                        "sources/"
                    ]))
                }),
            )
            .route("/x-nmos/node/:api/self", get(get_self))
            .route("/x-nmos/node/:api/devices/", get(get_devices))
            .route("/x-nmos/node/:api/devices/:id", get(get_device))
            .route("/x-nmos/node/:api/receivers/", get(get_receivers))
            .route("/x-nmos/node/:api/receivers/:id", get(get_receiver))
            .route("/x-nmos/node/:api/senders/", get(get_senders))
            .route("/x-nmos/node/:api/senders/:id", get(get_sender))
            .route("/x-nmos/node/:api/sources/", get(get_sources))
            .route("/x-nmos/node/:api/sources/:id", get(get_source))
            .route("/x-nmos/node/:api/flows/", get(get_flows))
            .route("/x-nmos/node/:api/flows/:id", get(get_flow))
            .fallback(fallback_handler.into_service())
            .layer(Extension(model));

        Self { router }
    }
}

async fn fallback_handler(OriginalUri(uri): OriginalUri) -> ServiceError {
    ServiceError::new(
        StatusCode::NOT_FOUND,
        Some(format!("No such path: {}", uri)),
    )
}

impl Service<Request<Body>> for NmosService {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.router.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        Box::pin(self.router.call(req))
    }
}
