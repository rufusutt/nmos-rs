mod device;
mod flow;
mod node;
mod receiver;
mod sender;
mod source;

use core::fmt;

#[derive(Debug)]
pub enum Format {
    Video,
    Audio,
    Data,
}

#[derive(Debug)]
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

pub trait Resource {
    type JsonType;

    fn to_json(&self) -> Self::JsonType;
}

pub use device::{Device, DeviceBuilder};
pub use flow::{Flow, FlowBuilder};
pub use node::{Node, NodeBuilder};
pub use receiver::{Receiver, ReceiverBuilder};
pub use sender::{Sender, SenderBuilder};
pub use source::{Source, SourceBuilder};

// pub trait Resource {
//     type ResourceType;
// }

// #[derive(Debug)]
// pub struct Resource {
//     api: String,
//     downgrade_api: String,
//     subresources: HashSet<Uuid>,
//     created: DateTime<Utc>,
//     updated: DateTime<Utc>,
//     data: String,
// }
