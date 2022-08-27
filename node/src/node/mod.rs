mod event_handler;
mod mdns;

use std::thread;
use std::time::Duration;
use std::{collections::BinaryHeap, sync::Arc};

use axum::http::Method;
use axum::Server;
pub use event_handler::EventHandler;
use mdns::MdnsContext;
use nmos_rs_model::resource;
use nmos_rs_model::{resource::ResourceBundle, Model};
use tokio::sync::mpsc;
use tower::make::Shared;
use tower::ServiceBuilder;
use tower_http::cors::{self, CorsLayer};
use tracing::info;

use crate::{
    error::Result,
    node::mdns::{NmosMdnsConfig, NmosMdnsEvent, NmosMdnsRegistry},
    service::NmosService,
};

pub struct NodeBuilder {
    event_handler: Option<Arc<dyn EventHandler>>,
    resource_bundle: ResourceBundle,
}

impl NodeBuilder {
    pub fn new() -> Self {
        Self {
            event_handler: None,
            resource_bundle: Default::default(),
        }
    }

    pub fn from_resources(resource_bundle: ResourceBundle) -> Self {
        Self {
            event_handler: None,
            resource_bundle,
        }
    }

    pub fn event_handler<H: EventHandler + 'static>(mut self, event_handler: H) -> Self {
        self.event_handler = Some(Arc::new(event_handler));

        self
    }

    pub fn node(&mut self, node: resource::Node) {
        self.resource_bundle.insert_node(node)
    }

    pub fn device(&mut self, device: resource::Device) {
        self.resource_bundle.insert_device(device)
    }

    pub fn source(&mut self, source: resource::Source) {
        self.resource_bundle.insert_source(source)
    }

    pub fn flow(&mut self, flow: resource::Flow) {
        self.resource_bundle.insert_flow(flow)
    }

    pub fn sender(&mut self, sender: resource::Sender) {
        self.resource_bundle.insert_sender(sender)
    }

    pub fn receiver(&mut self, receiver: resource::Receiver) {
        self.resource_bundle.insert_receiver(receiver)
    }

    pub async fn build(self) -> Node {
        // Create nmos model
        let model = Model::from_resources(self.resource_bundle);

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

    pub fn builder_from_resources(resource_bundle: ResourceBundle) -> NodeBuilder {
        NodeBuilder::from_resources(resource_bundle)
    }

    pub async fn start(self) -> Result<()> {
        info!("Starting nmos-rs node");

        // Channel for receiving MDNS events
        let (tx, mut rx) = mpsc::unbounded_channel();

        let mdns_thread = thread::spawn(move || {
            // Create context
            let mut context = MdnsContext::new(&NmosMdnsConfig {}, tx);

            let poller = context.start();

            loop {
                // Poll every 100 ms
                poller.poll();
                thread::sleep(Duration::from_millis(100));
            }
        });

        let mut registries = BinaryHeap::new();

        let mdns_receiver = async {
            while let Some(event) = rx.recv().await {
                if let NmosMdnsEvent::Discovery(_, Ok(discovery)) = event {
                    let mdns_registry = NmosMdnsRegistry::parse(&discovery);
                    registries.push(mdns_registry);
                }
            }
        };

        // Create server
        let app = ServiceBuilder::new()
            .layer(
                CorsLayer::new()
                    .allow_methods([Method::GET, Method::POST])
                    .allow_origin(cors::Any),
            )
            .service(self.service);

        let addr = ([0, 0, 0, 0], 3000).into();
        let server = Server::bind(&addr).serve(Shared::new(app));

        tokio::select! {
            _ = mdns_receiver => {}
            _ = server => {}
        };

        Ok(())
    }
}
