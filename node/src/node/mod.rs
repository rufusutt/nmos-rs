mod event_handler;
mod mdns;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use axum::http::Method;
use axum::Server;
pub use event_handler::EventHandler;
use mdns::MdnsContext;
use nmos_rs_model::{resource, Model};
use tokio::sync::mpsc;
use tower::make::Shared;
use tower::ServiceBuilder;
use tower_http::cors::{self, CorsLayer};
use tracing::info;

use crate::{error::Result, service::NmosService};

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

    pub async fn build(self) -> Node {
        // Create mdns context
        // let mdns_context = MdnsContext::new();

        // Create nmos model
        let mut model = Model::new();

        // Create new node
        let node = resource::NodeBuilder::new("Test").build();
        let device = resource::DeviceBuilder::new(&node, "devicetype").build();
        let receiver = resource::ReceiverBuilder::new(
            &device,
            resource::Format::Video,
            resource::Transport::RtpMulticast,
        )
        .build();

        model.insert_node(node).await;
        model.insert_device(device).await;
        model.insert_receiver(receiver).await;

        // Wrap model in Arc
        let model = Arc::new(model);

        // Make service
        let service = NmosService::new(model.clone());

        Node {
            event_handler: self.event_handler,
            model,
            service,
        }
    }
}

pub struct Node {
    event_handler: Option<Arc<dyn EventHandler>>,
    model: Arc<Model>,
    service: NmosService,
}

impl Node {
    pub fn builder() -> NodeBuilder {
        NodeBuilder::new()
    }

    pub async fn start(self) -> Result<()> {
        info!("Starting nmos-rs node");

        // Channel to get receivers back from thread
        let (tx, mut rx) = mpsc::channel(1);

        let mdns_thread = thread::spawn(move || {
            let mut context = MdnsContext::new();

            // Get receivers
            let receivers = context.receivers();
            tx.blocking_send(receivers).unwrap();

            let poller = context.start();

            loop {
                // Poll every 100 ms
                poller.poll();
                thread::sleep(Duration::from_millis(100));
            }
        });

        let mdns_receivers = rx.recv().await;

        // Create server
        let app = ServiceBuilder::new()
            .layer(
                CorsLayer::new()
                    .allow_methods([Method::GET, Method::POST])
                    .allow_origin(cors::Any),
            )
            .service(self.service);

        let addr = ([0, 0, 0, 0], 3000).into();
        Server::bind(&addr)
            .serve(Shared::new(app))
            .await
            .expect("Server error");

        // service.await;

        // let mut mdns_receivers = None;

        // thread::spawn(|| {
        //     let (mdns_context, mdns_receivers) = MdnsContext::new();

        //     mdns_context.poll();
        // });

        // let t = mdns_receivers.register_rx.recv().await;

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
