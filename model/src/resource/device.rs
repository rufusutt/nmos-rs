use std::fmt;

use nmos_schema::is_04;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    resource::Node,
    version::{is_04::V1_0, APIVersion},
};

use super::{ResourceCore, ResourceCoreBuilder};

#[derive(Debug, Clone, Copy)]
pub enum DeviceType {
    Generic,
    Pipeline,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceType::Generic => write!(f, "urn:x-nmos:device:generic"),
            DeviceType::Pipeline => write!(f, "urn:x-nmos:device:pipeline"),
        }
    }
}

pub struct DeviceBuilder {
    core: ResourceCoreBuilder,
    type_: DeviceType,
    node_id: Uuid,
}

impl DeviceBuilder {
    pub fn new<S: Into<String>>(label: S, node: &Node, device_type: DeviceType) -> DeviceBuilder {
        DeviceBuilder {
            core: ResourceCoreBuilder::new(label),
            type_: device_type,
            node_id: node.core.id,
        }
    }

    pub fn build(self) -> Device {
        Device {
            core: self.core.build(),
            type_: self.type_,
            node_id: self.node_id,
            senders: Vec::new(),
            receivers: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Device {
    pub core: ResourceCore,
    pub type_: DeviceType,
    pub node_id: Uuid,
    pub senders: Vec<Uuid>,
    pub receivers: Vec<Uuid>,
}

impl Device {
    pub fn builder<S: Into<String>>(label: S, node: &Node, device_type: DeviceType) -> DeviceBuilder {
        DeviceBuilder::new(label, node, device_type)
    }

    pub fn to_json(&self, api: &APIVersion) -> DeviceJson {
        match *api {
            V1_0 => {
                // Senders
                let senders = self.senders.iter().map(|s| s.to_string()).collect();

                // Receivers
                let receivers = self.receivers.iter().map(|r| r.to_string()).collect();

                DeviceJson::V1_0(is_04::v1_0_x::Device {
                    id: self.core.id.to_string(),
                    version: self.core.version.to_string(),
                    label: self.core.label.clone(),
                    type_: self.type_.to_string(),
                    node_id: self.node_id.to_string(),
                    senders,
                    receivers,
                })
            }
            _ => panic!("Unsupported API"),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum DeviceJson {
    V1_0(is_04::v1_0_x::Device),
}
