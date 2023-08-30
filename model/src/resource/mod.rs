use std::{collections::BTreeMap, fmt};

use uuid::Uuid;

pub use device::{Device, DeviceBuilder, DeviceJson, DeviceType};
pub use flow::{Flow, FlowBuilder, FlowJson};
pub use node::{Node, NodeBuilder, NodeJson, NodeService};
pub use receiver::{Receiver, ReceiverBuilder, ReceiverJson};
pub use sender::{Sender, SenderBuilder, SenderJson};
pub use source::{Source, SourceBuilder, SourceJson};

use crate::tai::TaiTime;

mod device;
mod flow;
mod node;
mod receiver;
mod sender;
mod source;

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Video,
    Audio,
    Data,
}

#[derive(Debug, Clone, Copy)]
pub enum Transport {
    Rtp,
    RtpUnicast,
    RtpMulticast,
    Dash,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Format::Video => write!(f, "urn:x-nmos:format:video"),
            Format::Audio => write!(f, "urn:x-nmos:format:audio"),
            Format::Data => write!(f, "urn:x-nmos:format:data"),
        }
    }
}

impl fmt::Display for Transport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Transport::Rtp => write!(f, "urn:x-nmos:transport:rtp"),
            Transport::RtpUnicast => write!(f, "urn:x-nmos:transport:rtp.ucast"),
            Transport::RtpMulticast => write!(f, "urn:x-nmos:transport:rtp.mcast"),
            Transport::Dash => write!(f, "urn:x-nmos:transport:dash"),
        }
    }
}

#[derive(Debug)]
#[must_use]
pub struct ResourceCoreBuilder {
    pub label: String,
    pub description: Option<String>,
    pub tags: BTreeMap<String, Vec<String>>,
}

impl ResourceCoreBuilder {
    pub fn new<S: Into<String>>(label: S) -> Self {
        Self {
            label: label.into(),
            description: None,
            tags: BTreeMap::new(),
        }
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn tag<S, V>(mut self, key: S, values: V) -> Self
    where
        S: Into<String>,
        V: IntoIterator<Item = S>,
    {
        let values: Vec<String> = values.into_iter().map(Into::into).collect();

        self.tags.insert(key.into(), values);
        self
    }

    #[must_use]
    pub fn build(self) -> ResourceCore {
        ResourceCore {
            id: Uuid::new_v4(),
            version: TaiTime::now(),
            label: self.label,
            description: self.description.unwrap_or_default(),
            tags: self.tags,
        }
    }
}

#[derive(Debug)]
pub struct ResourceCore {
    pub id: Uuid,
    pub version: TaiTime,
    pub label: String,
    pub description: String,
    pub tags: BTreeMap<String, Vec<String>>,
}

impl ResourceCore {
    pub fn builder<S: Into<String>>(label: S) -> ResourceCoreBuilder {
        ResourceCoreBuilder::new(label)
    }
}

#[derive(Debug, Default)]
pub struct ResourceBundle {
    pub(crate) nodes: Vec<Node>,
    pub(crate) devices: Vec<Device>,
    pub(crate) sources: Vec<Source>,
    pub(crate) flows: Vec<Flow>,
    pub(crate) senders: Vec<Sender>,
    pub(crate) receivers: Vec<Receiver>,
}

impl ResourceBundle {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn insert_device(&mut self, device: Device) {
        self.devices.push(device);
    }

    pub fn insert_source(&mut self, source: Source) {
        self.sources.push(source);
    }

    pub fn insert_flow(&mut self, flow: Flow) {
        self.flows.push(flow);
    }

    pub fn insert_sender(&mut self, sender: Sender) {
        self.senders.push(sender);
    }

    pub fn insert_receiver(&mut self, receiver: Receiver) {
        self.receivers.push(receiver);
    }
}
