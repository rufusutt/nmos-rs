mod event_handler;
mod mdns;
mod service;

pub use event_handler::EventHandler;
use mdns::MdnsContext;
use service::MakeNodeServce;
use tracing::info;

use crate::error::Result;

use nmos_schema::is_04::v1_0_x::RegistrationApiBase;

use hyper::client::HttpConnector;
use hyper::server::conn::AddrIncoming;
use hyper::{Client, Server, Uri};
use zeroconf::browser::TMdnsBrowser;
use zeroconf::event_loop::TEventLoop;
use zeroconf::{MdnsBrowser, ServiceDiscovery, ServiceType};

use std::any::Any;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct NodeBuilder {
    event_handler: Option<Arc<dyn EventHandler>>,
}

impl NodeBuilder {
    pub fn new() -> Self {
        Self {
            event_handler: None,
        }
    }

    pub fn event_handler<H: EventHandler + 'static>(mut self, event_handler: H) -> Self {
        self.event_handler = Some(Arc::new(event_handler));

        self
    }

    pub fn build(self) -> Node {
        // Create mdns context
        let mdns_context = MdnsContext::new();

        // Create server
        let addr = ([127, 0, 0, 1], 3000).into();
        let make_service = MakeNodeServce;
        let server = Server::bind(&addr).serve(make_service);

        // Create client
        let client = Client::new();

        Node {
            event_handler: self.event_handler,
            server,
            client,
        }
    }
}

pub struct Node {
    event_handler: Option<Arc<dyn EventHandler>>,
    server: Server<AddrIncoming, MakeNodeServce>,
    client: Client<HttpConnector>,
}

impl Node {
    pub fn builder() -> NodeBuilder {
        NodeBuilder::new()
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("Starting nmos-rs node");

        let (mdns_context, mdns_receivers) = MdnsContext::new();

        thread::spawn(|| {
            mdns_context.poll();
        });

        let t = mdns_receivers.register_rx.recv().await;

        // // Create browser
        // let mut browser = MdnsBrowser::new(ServiceType::new("nmos-register", "tcp").unwrap());

        // browser.set_service_discovered_callback(Box::new(Self::on_service_discovered));
        // browser.set_context(Box::new(self.mdns_context.clone()));

        // let event_loop = browser.browse_services().unwrap();

        // info!("Discovering registries");

        // Scan for duration
        // let start = Instant::now();
        // loop {
        //     event_loop.poll(Duration::from_secs(0)).unwrap();
        //     if Instant::now().duration_since(start) > Duration::from_secs(1) {
        //         break;
        //     }
        // }


        // let context = self.mdns_context.lock().unwrap();
        // let service = context.services.first().expect("No services");
        // let uri = Uri::builder()
        //     .scheme("http")
        //     .authority(format!("{}:{}", service.address(), service.port()))
        //     .path_and_query("/x-nmos/registration/v1.0")
        //     .build()
        //     .unwrap();

        // let res = self.client.get(uri).await?;

        // // res.body();

        // let buf = hyper::body::to_bytes(res).await?;

        // let base: RegistrationApiBase = serde_json::from_slice(&buf).unwrap();

        // dbg!(base);

        // let server = &mut self.server;

        // server.await.map_err(|e| e.into())
        Ok(())
    }
}
