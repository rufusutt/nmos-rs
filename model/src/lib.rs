pub mod resource;
pub mod tai;
pub mod version;

use std::collections::HashMap;

use resource::{Device, Flow, Node, Receiver, Sender, Source};
use tokio::sync::{RwLock, RwLockReadGuard};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Model {
    // IS-04 resources
    nodes: RwLock<HashMap<Uuid, Node>>,
    devices: RwLock<HashMap<Uuid, Device>>,
    receivers: RwLock<HashMap<Uuid, Receiver>>,
    senders: RwLock<HashMap<Uuid, Sender>>,
    sources: RwLock<HashMap<Uuid, Source>>,
    flows: RwLock<HashMap<Uuid, Flow>>,
}

impl Model {
    pub fn new() -> Self {
        Default::default()
    }

    // Get nodes
    pub async fn nodes<'a>(&'a self) -> RwLockReadGuard<'a, HashMap<Uuid, Node>> {
        self.nodes.read().await
    }

    // Get devices
    pub async fn devices<'a>(&'a self) -> RwLockReadGuard<'a, HashMap<Uuid, Device>> {
        self.devices.read().await
    }

    // Get receivers
    pub async fn receivers<'a>(&'a self) -> RwLockReadGuard<'a, HashMap<Uuid, Receiver>> {
        self.receivers.read().await
    }

    // Get senders
    pub async fn senders<'a>(&'a self) -> RwLockReadGuard<'a, HashMap<Uuid, Sender>> {
        self.senders.read().await
    }

    // Get sources
    pub async fn sources<'a>(&'a self) -> RwLockReadGuard<'a, HashMap<Uuid, Source>> {
        self.sources.read().await
    }

    // Get flows
    pub async fn flows<'a>(&'a self) -> RwLockReadGuard<'a, HashMap<Uuid, Flow>> {
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
