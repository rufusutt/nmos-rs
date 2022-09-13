use nmos_schema::is_04;
use serde::Serialize;

use crate::version::{is_04::V1_0, APIVersion};

use super::{ResourceCore, ResourceCoreBuilder};

#[derive(Debug)]
pub struct NodeService {
    pub href: String,
    pub type_: String,
}

pub struct NodeBuilder {
    core: ResourceCoreBuilder,
    href: String,
    hostname: Option<String>,
    services: Vec<NodeService>,
}

impl NodeBuilder {
    pub fn new<S: Into<String>>(label: S, href: S) -> NodeBuilder {
        NodeBuilder {
            core: ResourceCoreBuilder::new(label),
            href: href.into(),
            hostname: None,
            services: Vec::new(),
        }
    }

    pub fn with_service(mut self, service: NodeService) -> NodeBuilder {
        self.services.push(service);
        self
    }

    pub fn build(self) -> Node {
        Node {
            core: self.core.build(),
            href: self.href,
            hostname: self.hostname,
            services: self.services,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub core: ResourceCore,
    pub href: String,
    pub hostname: Option<String>,
    pub services: Vec<NodeService>,
}

impl Node {
    pub fn builder<S: Into<String>>(label: S, href: S) -> NodeBuilder {
        NodeBuilder::new(label, href)
    }

    pub fn to_json(&self, api: &APIVersion) -> NodeJson {
        match *api {
            V1_0 => {
                let services = self
                    .services
                    .iter()
                    .map(|service| is_04::v1_0_x::NodeItemServices {
                        href: service.href.clone(),
                        type_: service.type_.clone(),
                    })
                    .collect();

                NodeJson::V1_0(is_04::v1_0_x::Node {
                    id: self.core.id.to_string(),
                    version: self.core.version.to_string(),
                    label: self.core.label.clone(),
                    href: self.href.clone(),
                    hostname: self.hostname.clone(),
                    caps: Default::default(),
                    services,
                })
            }
            _ => panic!("Unsupported API"),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum NodeJson {
    V1_0(is_04::v1_0_x::Node),
}
