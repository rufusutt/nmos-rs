pub mod resource;
pub mod tai;
pub mod version;

use std::collections::HashMap;

use resource::{Device, Flow, Node, Receiver, ResourceBundle, Sender, Source};
use tokio::sync::{RwLock, RwLockReadGuard};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Model {
    // IS-04 resources
    nodes: RwLock<HashMap<Uuid, Node>>,
    devices: RwLock<HashMap<Uuid, Device>>,
    sources: RwLock<HashMap<Uuid, Source>>,
    flows: RwLock<HashMap<Uuid, Flow>>,
    senders: RwLock<HashMap<Uuid, Sender>>,
    receivers: RwLock<HashMap<Uuid, Receiver>>,
}

impl Model {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_resources(resource_bundle: ResourceBundle) -> Self {
        // Fold each resource vec into a hashmap
        let nodes = resource_bundle
            .nodes
            .into_iter()
            .fold(HashMap::new(), |mut map, node| {
                map.insert(node.id, node);
                map
            });

        let devices =
            resource_bundle
                .devices
                .into_iter()
                .fold(HashMap::new(), |mut map, device| {
                    map.insert(device.id, device);
                    map
                });

        let sources =
            resource_bundle
                .sources
                .into_iter()
                .fold(HashMap::new(), |mut map, source| {
                    map.insert(source.id, source);
                    map
                });

        let flows = resource_bundle
            .flows
            .into_iter()
            .fold(HashMap::new(), |mut map, flow| {
                map.insert(flow.id, flow);
                map
            });

        let senders =
            resource_bundle
                .senders
                .into_iter()
                .fold(HashMap::new(), |mut map, sender| {
                    map.insert(sender.id, sender);
                    map
                });

        let receivers =
            resource_bundle
                .receivers
                .into_iter()
                .fold(HashMap::new(), |mut map, receiver| {
                    map.insert(receiver.id, receiver);
                    map
                });

        Self {
            nodes: RwLock::new(nodes),
            devices: RwLock::new(devices),
            sources: RwLock::new(sources),
            flows: RwLock::new(flows),
            senders: RwLock::new(senders),
            receivers: RwLock::new(receivers),
        }
    }

    // Get nodes
    pub async fn nodes(&self) -> RwLockReadGuard<'_, HashMap<Uuid, Node>> {
        self.nodes.read().await
    }

    // Get devices
    pub async fn devices(&self) -> RwLockReadGuard<'_, HashMap<Uuid, Device>> {
        self.devices.read().await
    }

    // Get receivers
    pub async fn receivers(&self) -> RwLockReadGuard<'_, HashMap<Uuid, Receiver>> {
        self.receivers.read().await
    }

    // Get senders
    pub async fn senders(&self) -> RwLockReadGuard<'_, HashMap<Uuid, Sender>> {
        self.senders.read().await
    }

    // Get sources
    pub async fn sources(&self) -> RwLockReadGuard<'_, HashMap<Uuid, Source>> {
        self.sources.read().await
    }

    // Get flows
    pub async fn flows(&self) -> RwLockReadGuard<'_, HashMap<Uuid, Flow>> {
        self.flows.read().await
    }

    pub async fn insert_node(&mut self, node: Node) -> Option<()> {
        let mut nodes = self.nodes.write().await;
        nodes.insert(node.id, node);

        Some(())
    }

    pub async fn insert_device(&mut self, device: Device) -> Option<()> {
        // Check node id in model
        let nodes = self.nodes.read().await;
        if !nodes.contains_key(&device.node_id) {
            return None;
        }

        let mut devices = self.devices.write().await;
        devices.insert(device.id, device);

        Some(())
    }

    pub async fn insert_receiver(&mut self, receiver: Receiver) -> Option<()> {
        // Check device id in model
        let devices = self.devices.read().await;
        if !devices.contains_key(&receiver.device_id) {
            return None;
        }

        let mut receivers = self.receivers.write().await;
        receivers.insert(receiver.id, receiver);

        Some(())
    }
}
