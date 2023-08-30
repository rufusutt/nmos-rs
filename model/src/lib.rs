pub mod resource;
pub mod tai;
pub mod version;

use std::collections::HashMap;

use resource::{Device, Flow, Node, Receiver, ResourceBundle, Sender, Source};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Model {
    // IS-04 resources
    pub nodes: HashMap<Uuid, Node>,
    pub devices: HashMap<Uuid, Device>,
    pub sources: HashMap<Uuid, Source>,
    pub flows: HashMap<Uuid, Flow>,
    pub senders: HashMap<Uuid, Sender>,
    pub receivers: HashMap<Uuid, Receiver>,
}

impl Model {
    #[must_use]
    pub fn new() -> Self {
        Model::default()
    }

    #[must_use]
    pub fn from_resources(resource_bundle: ResourceBundle) -> Self {
        // Fold each resource vec into a hashmap
        let nodes = resource_bundle
            .nodes
            .into_iter()
            .fold(HashMap::new(), |mut map, node| {
                map.insert(node.core.id, node);
                map
            });

        let devices =
            resource_bundle
                .devices
                .into_iter()
                .fold(HashMap::new(), |mut map, device| {
                    map.insert(device.core.id, device);
                    map
                });

        let sources =
            resource_bundle
                .sources
                .into_iter()
                .fold(HashMap::new(), |mut map, source| {
                    map.insert(source.core.id, source);
                    map
                });

        let flows = resource_bundle
            .flows
            .into_iter()
            .fold(HashMap::new(), |mut map, flow| {
                map.insert(flow.core.id, flow);
                map
            });

        let senders =
            resource_bundle
                .senders
                .into_iter()
                .fold(HashMap::new(), |mut map, sender| {
                    map.insert(sender.core.id, sender);
                    map
                });

        let receivers =
            resource_bundle
                .receivers
                .into_iter()
                .fold(HashMap::new(), |mut map, receiver| {
                    map.insert(receiver.core.id, receiver);
                    map
                });

        Self {
            nodes,
            devices,
            sources,
            flows,
            senders,
            receivers,
        }
    }

    pub async fn insert_node(&mut self, node: Node) -> Option<()> {
        self.nodes.insert(node.core.id, node);

        Some(())
    }

    pub async fn insert_device(&mut self, device: Device) -> Option<()> {
        // Check node id in model
        if !self.nodes.contains_key(&device.node_id) {
            return None;
        }

        self.devices.insert(device.core.id, device);

        Some(())
    }

    pub async fn insert_receiver(&mut self, receiver: Receiver) -> Option<()> {
        // Check device id in model
        if !self.devices.contains_key(&receiver.device_id) {
            return None;
        }

        self.receivers.insert(receiver.core.id, receiver);

        Some(())
    }
}
