use std::{collections::BinaryHeap, sync::Arc, thread, time::Duration};

use axum::{http::Method, Server};
pub use event_handler::EventHandler;
use mdns::MdnsContext;
use nmos_model::{resource::ResourceBundle, Model};
use tokio::{
    runtime::Runtime,
    sync::{mpsc, Mutex},
};
use tower::{make::Shared, ServiceBuilder};
use tower_http::cors::{self, CorsLayer};
use tracing::{error, info};

mod api;
mod error;
mod event_handler;
mod mdns;

pub use async_trait::async_trait;
pub use error::Error as NmosError;

use api::{NodeApi, RegistrationApi};
use mdns::{NmosMdnsConfig, NmosMdnsEvent, NmosMdnsRegistry};

#[derive(Default)]
pub struct NodeBuilder {
    model: Model,
    event_handler: Option<Arc<dyn EventHandler>>,
}

impl NodeBuilder {
    pub fn new(model: Model) -> Self {
        Self {
            model,
            event_handler: None,
        }
    }

    pub fn from_resources(resource_bundle: ResourceBundle) -> Self {
        Self {
            model: Model::from_resources(resource_bundle),
            event_handler: None,
        }
    }

    pub fn event_handler<H: EventHandler + 'static>(mut self, event_handler: H) -> Self {
        self.event_handler = Some(Arc::new(event_handler));
        self
    }

    pub fn build(self) -> Node {
        // Wrap model in Arc
        let model = Arc::new(self.model);

        // Make service
        let service = NodeApi::new(model.clone());

        Node {
            _event_handler: self.event_handler,
            model,
            service,
        }
    }
}

pub struct Node {
    _event_handler: Option<Arc<dyn EventHandler>>,
    model: Arc<Model>,
    service: NodeApi,
}

impl Node {
    pub fn builder(model: Model) -> NodeBuilder {
        NodeBuilder::new(model)
    }

    pub fn builder_from_resources(resource_bundle: ResourceBundle) -> NodeBuilder {
        NodeBuilder::from_resources(resource_bundle)
    }

    pub fn model(&self) -> Arc<Model> {
        self.model.clone()
    }

    pub async fn start(self) -> error::Result<()> {
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
            // Create http client
            let client = reqwest::Client::new();

            loop {
                // Wait for registry discovery
                tokio::time::sleep(Duration::from_secs(5)).await;

                // Try and get highest priority registry
                let registry = {
                    let mut registries = registries.lock().await;
                    match registries.pop() {
                        Some(r) => r,
                        None => continue,
                    }
                };

                // Attempt to register
                match RegistrationApi::register_resources(&client, self.model.clone(), &registry)
                    .await
                {
                    Ok(_) => info!("Registration successful"),
                    Err(err) => {
                        error!("Failed to register with registry: {}", err);
                        continue;
                    }
                }

                // Get heartbeat endpoint from node id
                let heartbeat_url = {
                    let nodes = self.model.nodes().await;
                    let node_id = *nodes.iter().next().unwrap().0;

                    let base = &registry.url.join("v1.0/").unwrap();
                    base.join(&format!("health/nodes/{}", node_id)).unwrap()
                };

                // Send heartbeat every 5 seconds
                loop {
                    match client.post(heartbeat_url.clone()).send().await {
                        Ok(res) => {
                            if !res.status().is_success() {
                                error!("Heartbeat error");
                                break;
                            }
                        }
                        Err(err) => {
                            error!("Failed to send heartbeat: {}", err);
                            break;
                        }
                    }
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        };

        tokio::select! {
            _ = mdns_receiver => {}
            _ = http_server => {}
            _ = registration => {}
        };

        Ok(())
    }

    pub fn start_blocking(self) -> error::Result<()> {
        let rt = Runtime::new().expect("Unable to create Tokio runtime");
        rt.block_on(self.start())
    }
}
