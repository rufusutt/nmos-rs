mod event_handler;
mod mdns;

use std::thread;
use std::time::Duration;
use std::{collections::BinaryHeap, sync::Arc};

use axum::http::Method;
use axum::Server;
pub use event_handler::EventHandler;
use mdns::MdnsContext;
use nmos_rs_model::{
    resource::{self, ResourceBundle},
    Model,
};
use tokio::sync::{mpsc, Mutex};
use tower::make::Shared;
use tower::ServiceBuilder;
use tower_http::cors::{self, CorsLayer};
use tracing::{error, info};

use crate::{
    error::Result,
    node::mdns::{NmosMdnsConfig, NmosMdnsEvent, NmosMdnsRegistry},
    service::NmosService,
};

#[derive(Default)]
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

    pub fn build(self) -> Node {
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

    pub fn model(&self) -> Arc<Model> {
        self.model.clone()
    }

    async fn register_resources(model: Arc<Model>, registry: &NmosMdnsRegistry) {
        // let base = &registry.url.join("v1.0/").unwrap();

        // info!("Attempting to register with {}", base);

        // // Resource endpoint
        // let resource = &base.join("resource").unwrap();

        // // for node in nodes {
        // //     Self::register_node(base, node)
        // // }

        // let nodes = model.nodes().await;
        // let node = nodes.iter().next().unwrap().1;

        // let node_json = is_04::v1_0_x::registrationapi::ResourcePostRequestJsonNode {
        //     data: Some(node.to_json()),
        //     type_: Some(String::from("node")),
        // };

        // let post_request =
        //     is_04::v1_0_x::registrationapi::ResourcePostRequestJson::Variant0(node_json);

        // let ret = client
        //     .post(resource.clone())
        //     .json(&post_request)
        //     .send()
        //     .await;

        // info!("{:?}", ret);
    }

    pub async fn start(self) -> Result<()> {
        info!("Starting nmos-rs node");

        // Channel for receiving MDNS events
        let (tx, mut rx) = mpsc::unbounded_channel();

        // Keep discovered registries in a priority queue
        let registries = Arc::new(Mutex::new(BinaryHeap::new()));

        // MDNS must run on its own thread
        // Events are sent back to the Tokio runtime
        thread::spawn(move || {
            // Create context
            let mut context = MdnsContext::new(&NmosMdnsConfig {}, tx.clone());
            let poller = context.start();

            loop {
                // Check event channel is still valid
                if tx.is_closed() {
                    break;
                }

                // Poll every 100 ms
                poller.poll();
                thread::sleep(Duration::from_millis(100));
            }
        });

        // Receive MDNS events in "main thread"
        let mdns_receiver = async {
            let registries = registries.clone();

            while let Some(event) = rx.recv().await {
                if let NmosMdnsEvent::Discovery(_, Ok(discovery)) = event {
                    if let Some(registry) = NmosMdnsRegistry::parse(&discovery) {
                        registries.lock().await.push(registry);
                    }
                }
            }
        };

        // Create HTTP service
        let app = ServiceBuilder::new()
            .layer(
                CorsLayer::new()
                    .allow_methods([Method::GET, Method::POST])
                    .allow_origin(cors::Any),
            )
            .service(self.service);

        let addr = ([0, 0, 0, 0], 3000).into();
        let http_server = Server::bind(&addr).serve(Shared::new(app));

        // Registry connection thread
        let registration = async {
            // Initial wait for registry discovery
            tokio::time::sleep(Duration::from_secs(5)).await;

            // Create http client
            let client = reqwest::Client::new();

            let registries = registries.lock().await;
            if registries.is_empty() {
                error!("Failed to discover a registry");
                return;
            }

            for registry in registries.iter() {
                Self::register_resources(self.model.clone(), registry).await;
            }

            tokio::time::sleep(Duration::from_millis(68719476734)).await;
        };

        tokio::select! {
            _ = mdns_receiver => {}
            _ = http_server => {}
            // _ = registration => {}
        };

        Ok(())
    }
}
