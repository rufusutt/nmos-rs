mod event_handler;
mod service;

pub use event_handler::EventHandler;
use service::NodeService;

use hyper::server::conn::AddrIncoming;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Request, Body, Response, StatusCode, Method, Server};

use crate::error::Result;

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context as FutContext, Poll};

use futures::future::BoxFuture;

pub struct NodeBuilder {
    fut: Option<BoxFuture<'static, Result<Node>>>,
    event_handler: Option<Arc<dyn EventHandler>>,
}

impl NodeBuilder {
    pub fn new() -> Self {
        Self {
            fut: None,
            event_handler: None,
        }
    }

    pub fn event_handler<H: EventHandler + 'static>(mut self, event_handler: H) -> Self {
        self.event_handler = Some(Arc::new(event_handler));

        self
    }
}

impl Future for NodeBuilder {
    type Output = Result<Node>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut FutContext<'_>) -> Poll<Self::Output> {
        if self.fut.is_none() {
            let event_handler = self.event_handler.take();

            // Create server
            let addr = ([127, 0, 0, 1], 3000).into();

            let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });
            let server = Server::bind(&addr).serve(NodeService {});

            self.fut = Some(Box::pin(async move {

                Ok(Node {
                    event_handler: event_handler,
                    server: server,
                })
            }));
        }

        self.fut.as_mut().unwrap().as_mut().poll(ctx)
    }
}

pub struct Node {
    pub event_handler: Option<Arc<dyn EventHandler>>,
    pub server: Service,
}

impl Node {
    pub fn builder() -> NodeBuilder {
        NodeBuilder::new()
    }

    pub async fn start(&mut self) -> Result<()> {
        let addr = ([127, 0, 0, 1], 3000).into();

        let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });
        let server = Server::bind(&addr).serve(service);

        Ok(())
    }
}


async fn echo(req: Request<Body>) -> std::result::Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from("Test"))),
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}