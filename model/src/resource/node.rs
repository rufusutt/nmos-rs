use nmos_rs_schema::is_04;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    tai::TaiTime,
    version::{is_04::V1_0, APIVersion},
};

#[derive(Debug)]
pub struct NodeService {
    pub href: String,
    pub type_: String,
}

pub struct NodeBuilder {
    label: Option<String>,
    href: String,
    hostname: Option<String>,
    services: Vec<NodeService>,
}

impl NodeBuilder {
    pub fn new<S: Into<String>>(href: S) -> NodeBuilder {
        NodeBuilder {
            label: None,
            href: href.into(),
            hostname: None,
            services: Vec::new(),
        }
    }

    pub fn label<S: Into<String>>(mut self, label: S) -> NodeBuilder {
        self.label = Some(label.into());
        self
    }

    pub fn with_service(mut self, service: NodeService) -> NodeBuilder {
        self.services.push(service);
        self
    }

    pub fn build(self) -> Node {
        Node {
            id: Uuid::new_v4(),
            version: TaiTime::now(),
            label: self.label.unwrap_or_default(),
            href: self.href,
            hostname: self.hostname,
            services: self.services,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub id: Uuid,
    pub version: TaiTime,
    pub label: String,
    pub href: String,
    pub hostname: Option<String>,
    pub services: Vec<NodeService>,
}

impl Node {
    pub fn builder<S: Into<String>>(href: S) -> NodeBuilder {
        NodeBuilder::new(href)
    }

    pub fn to_json(&self, api: &APIVersion) -> NodeJson {
        match *api {
            V1_0 => {
                let services = self
                    .services
                    .iter()
                    .map(|service| is_04::v1_0_x::NodeJsonItemServices {
                        href: service.href.clone(),
                        type_: service.type_.clone(),
                    })
                    .collect();

                NodeJson::V1_0(is_04::v1_0_x::NodeJson {
                    id: self.id.to_string(),
                    version: self.version.to_string(),
                    label: self.label.clone(),
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
    V1_0(is_04::v1_0_x::NodeJson),
}
