use hyper::{Method, StatusCode};
use std::task::Context;
use std::task::Poll;

use hyper::{service::Service, Body, Request, Response};

pub struct MakeNodeServce;

impl<T> Service<T> for MakeNodeServce {
    type Response = NodeService;
    type Error = std::io::Error;
    type Future = futures::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        futures::future::ok(NodeService)
    }
}

pub struct NodeService;

impl Service<Request<Body>> for NodeService {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = futures::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => futures::future::ok(Response::new(Body::from("Hello world"))),
            _ => {
                let mut not_found = Response::default();
                *not_found.status_mut() = StatusCode::NOT_FOUND;

                futures::future::ok(not_found)
            }
        }
    }
}
