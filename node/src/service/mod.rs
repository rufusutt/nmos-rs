mod node;
mod response;

use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;

use futures::Future;
use hyper::header::CONTENT_TYPE;
use hyper::{service::Service, Body, Method, Request, Response, StatusCode};
use tracing::{span, Level};

use nmos_rs_model::Model;

pub struct MakeNodeServce {
    model: Arc<Model>,
}

impl MakeNodeServce {
    pub fn new(model: Arc<Model>) -> Self {
        Self { model }
    }
}

impl<T> Service<T> for MakeNodeServce {
    type Response = NodeService;
    type Error = std::io::Error;
    type Future = futures::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        futures::future::ok(NodeService {
            model: self.model.clone(),
        })
    }
}

pub struct NodeService {
    model: Arc<Model>,
}

impl NodeService {
    async fn respond(
        req: Request<Body>,
        model: Arc<Model>,
    ) -> Result<Response<Body>, hyper::Error> {
        // Remove leading slash
        let mut path = req
            .uri()
            .path()
            .split_once('/')
            .unwrap_or(("", req.uri().path()))
            .1
            .to_owned();

        // Remove trailing slash if present
        if let Some('/') = path.chars().last() {
            path.pop();
        }

        // Split path string by remaining slashes
        let mut split_path = path.split('/');

        // Iterate through split path
        match (split_path.next(), req.method()) {
            // nmos request
            (Some("x-nmos"), _) => match (split_path.next(), req.method()) {
                // Node API
                (Some("node"), _) => node::respond(req, split_path, model).await,
                // Unknown path
                (Some(_), _) => response::not_found(),
                // GET x-nmos root
                (None, &Method::GET) => {
                    let body = Body::from(r#"["node/"]"#);

                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header(CONTENT_TYPE, "application/json")
                        .body(body)
                        .unwrap())
                }
                // Method not allowed
                (None, _) => response::method_not_allowed(),
            },
            // Unknown path
            (Some(_), _) => response::not_found(),
            // GET root
            (None, &Method::GET) => {
                let body = Body::from(r#"["x-manifest/","x-nmos/"]"#);

                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, "application/json")
                    .body(body)
                    .unwrap())
            }
            // Method not allowed
            (None, _) => response::method_not_allowed(),
        }
    }
}

impl Service<Request<Body>> for NodeService {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let span = span!(Level::INFO, "req");
        let _guard = span.enter();
        Box::pin(Self::respond(req, self.model.clone()))
    }
}
