use std::{collections::BinaryHeap, sync::Arc, thread, time::Duration};

use axum::{http::Method, Server};
pub use event_handler::EventHandler;
use mdns::MdnsContext;
use nmos_model::{
    resource::{self, ResourceBundle},
    Model,
};
use tokio::{
    runtime::Runtime,
    sync::{mpsc, Mutex},
};
use tower::{make::Shared, ServiceBuilder};
use tower_http::cors::{self, CorsLayer};
use tracing::{error, info};

mod error;
mod event_handler;
mod mdns;
mod service;

pub use async_trait::async_trait;
pub use error::Error as NmosError;

use mdns::{NmosMdnsConfig, NmosMdnsEvent, NmosMdnsRegistry};
use service::NmosService;

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
    pub fn builder(model: Model) -> NodeBuilder {
        NodeBuilder::new(model)
    }

    pub fn builder_from_resources(resource_bundle: ResourceBundle) -> NodeBuilder {
        NodeBuilder::from_resources(resource_bundle)
    }

    pub fn model(&self) -> Arc<Model> {
        self.model.clone()
    }

    async fn register_node(
        client: &reqwest::Client,
        url: &reqwest::Url,
        node: &resource::Node,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use nmos_model::version::is_04::V1_0;
        use nmos_schema::is_04::v1_0_x::{
            RegistrationapiResourcePostRequest, RegistrationapiResourcePostRequestHealthVariant0,
        };

        // TODO: Must find better way of representing multiple API
        // version in JSON. For now this will look like a mess.
        let node_json = match node.to_json(&V1_0) {
            resource::NodeJson::V1_0(json) => json,
        };

        // Construct POST request
        let node_post_request = RegistrationapiResourcePostRequestHealthVariant0 {
            data: Some(node_json),
            type_: Some(String::from("node")),
        };
        let post_request = RegistrationapiResourcePostRequest::Variant0(node_post_request);

        client.post(url.clone()).json(&post_request).send().await?;

        Ok(())
    }

    async fn register_device(
        client: &reqwest::Client,
        url: &reqwest::Url,
        device: &resource::Device,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use nmos_model::version::is_04::V1_0;
        use nmos_schema::is_04::v1_0_x::{
            RegistrationapiResourcePostRequest, RegistrationapiResourcePostRequestHealthVariant1,
        };

        let device_json = match device.to_json(&V1_0) {
            resource::DeviceJson::V1_0(json) => json,
        };
        let device_post_request = RegistrationapiResourcePostRequestHealthVariant1 {
            data: Some(device_json),
            type_: Some(String::from("device")),
        };
        let post_request = RegistrationapiResourcePostRequest::Variant1(device_post_request);

        client.post(url.clone()).json(&post_request).send().await?;

        Ok(())
    }

    async fn register_source(
        client: &reqwest::Client,
        url: &reqwest::Url,
        source: &resource::Source,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use nmos_model::version::is_04::V1_0;
        use nmos_schema::is_04::v1_0_x::{
            RegistrationapiResourcePostRequest, RegistrationapiResourcePostRequestHealthVariant4,
        };

        let source_json = match source.to_json(&V1_0) {
            resource::SourceJson::V1_0(json) => json,
        };
        let source_post_request = RegistrationapiResourcePostRequestHealthVariant4 {
            data: Some(source_json),
            type_: Some(String::from("source")),
        };
        let post_request = RegistrationapiResourcePostRequest::Variant4(source_post_request);

        client.post(url.clone()).json(&post_request).send().await?;

        Ok(())
    }

    async fn register_flow(
        client: &reqwest::Client,
        url: &reqwest::Url,
        flow: &resource::Flow,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use nmos_model::version::is_04::V1_0;
        use nmos_schema::is_04::v1_0_x::{
            RegistrationapiResourcePostRequest, RegistrationapiResourcePostRequestHealthVariant5,
        };

        let flow_json = match flow.to_json(&V1_0) {
            resource::FlowJson::V1_0(json) => json,
        };
        let flow_post_request = RegistrationapiResourcePostRequestHealthVariant5 {
            data: Some(flow_json),
            type_: Some(String::from("flow")),
        };
        let post_request = RegistrationapiResourcePostRequest::Variant5(flow_post_request);

        client.post(url.clone()).json(&post_request).send().await?;

        Ok(())
    }

    async fn register_sender(
        client: &reqwest::Client,
        url: &reqwest::Url,
        sender: &resource::Sender,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use nmos_model::version::is_04::V1_0;
        use nmos_schema::is_04::v1_0_x::{
            RegistrationapiResourcePostRequest, RegistrationapiResourcePostRequestHealthVariant2,
        };

        let sender_json = match sender.to_json(&V1_0) {
            resource::SenderJson::V1_0(json) => json,
        };
        let sender_post_request = RegistrationapiResourcePostRequestHealthVariant2 {
            data: Some(sender_json),
            type_: Some(String::from("sender")),
        };
        let post_request = RegistrationapiResourcePostRequest::Variant2(sender_post_request);

        client.post(url.clone()).json(&post_request).send().await?;

        Ok(())
    }

    async fn register_receiver(
        client: &reqwest::Client,
        url: &reqwest::Url,
        receiver: &resource::Receiver,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use nmos_model::version::is_04::V1_0;
        use nmos_schema::is_04::v1_0_x::{
            RegistrationapiResourcePostRequest, RegistrationapiResourcePostRequestHealthVariant3,
        };

        let receiver_json = match receiver.to_json(&V1_0) {
            resource::ReceiverJson::V1_0(json) => json,
        };
        let receiver_post_request = RegistrationapiResourcePostRequestHealthVariant3 {
            data: Some(receiver_json),
            type_: Some(String::from("receiver")),
        };
        let post_request = RegistrationapiResourcePostRequest::Variant3(receiver_post_request);

        client.post(url.clone()).json(&post_request).send().await?;

        Ok(())
    }

    async fn register_resources(
        client: &reqwest::Client,
        model: Arc<Model>,
        registry: &NmosMdnsRegistry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let base = &registry.url.join("v1.0/").unwrap();

        info!("Attempting to register with {}", base);

        // Resource endpoint
        let resource_url = &base.join("resource").unwrap();

        // Get node
        let nodes = model.nodes().await;
        let node = nodes.iter().next().unwrap().1;

        // Register resources in order
        Self::register_node(client, resource_url, node).await?;
        for (_, device) in model.devices().await.iter() {
            Self::register_device(client, resource_url, device).await?;
        }
        for (_, source) in model.sources().await.iter() {
            Self::register_source(client, resource_url, source).await?;
        }
        for (_, flow) in model.flows().await.iter() {
            Self::register_flow(client, resource_url, flow).await?;
        }
        for (_, sender) in model.senders().await.iter() {
            Self::register_sender(client, resource_url, sender).await?;
        }
        for (_, receiver) in model.receivers().await.iter() {
            Self::register_receiver(client, resource_url, receiver).await?;
        }

        Ok(())
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
                match Self::register_resources(&client, self.model.clone(), &registry).await {
                    Ok(_) => info!("Registration successful"),
                    Err(err) => {
                        error!("Failed to register with registry: {}", err);
                        continue;
                    }
                }

                // Get heartbeat endpoint from node id
                let heartbeat_url = {
                    let nodes = self.model.nodes().await;
                    let node_id = nodes.iter().next().unwrap().0.clone();

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
