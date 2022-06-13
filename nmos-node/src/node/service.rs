use std::pin::Pin;
use std::task::Poll;
use std::{future::Future, task::Context};

use hyper::{service::Service, Body, Request, Response};

pub struct NodeService {}

impl Service<Request<Body>> for NodeService {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        todo!()
    }
}
