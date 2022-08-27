use std::collections::BTreeMap;

use nmos_rs_schema::is_04;
use uuid::Uuid;

use super::{Device, Flow, Resource, Transport};
use crate::tai::TaiTime;

#[derive(Debug)]
pub struct Sender {
    pub id: Uuid,
    pub version: TaiTime,
    pub label: String,
    pub description: String,
    pub flow_id: Uuid,
    pub transport: Transport,
    pub tags: BTreeMap<String, Vec<String>>,
    pub device_id: Uuid,
    pub manifest_href: String,
}

impl Sender {
    pub fn builder(device: &Device, flow: &Flow, transport: Transport) -> SenderBuilder {
        SenderBuilder::new(device, flow, transport)
    }
}

pub struct SenderBuilder {
    pub label: Option<String>,
    pub description: Option<String>,
    pub flow_id: Uuid,
    pub transport: Transport,
    pub tags: BTreeMap<String, Vec<String>>,
    pub device_id: Uuid,
    pub manifest_href: Option<String>,
}

impl SenderBuilder {
    pub fn new(device: &Device, flow: &Flow, transport: Transport) -> SenderBuilder {
        SenderBuilder {
            label: None,
            description: None,
            flow_id: flow.id,
            transport,
            tags: Default::default(),
            device_id: device.id,
            manifest_href: None,
        }
    }

    pub fn label(mut self, label: String) -> SenderBuilder {
        self.label = Some(label);
        self
    }

    pub fn description(mut self, description: String) -> SenderBuilder {
        self.description = Some(description);
        self
    }

    pub fn build(self) -> Sender {
        Sender {
            id: Uuid::new_v4(),
            version: TaiTime::now(),
            label: self.label.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
            flow_id: self.flow_id,
            transport: self.transport,
            tags: self.tags,
            device_id: self.device_id,
            manifest_href: self.manifest_href.unwrap_or_default(),
        }
    }
}

impl Resource for Sender {
    type JsonType = is_04::v1_0_x::SenderJson;

    fn to_json(&self) -> Self::JsonType {
        let tags = if !self.tags.is_empty() {
            Some(
                self.tags
                    .iter()
                    .fold(BTreeMap::new(), |mut map, (key, array)| {
                        let value = serde_json::Value::from(array.clone());
                        map.insert(key.clone(), value);
                        map
                    }),
            )
        } else {
            None
        };

        is_04::v1_0_x::SenderJson {
            id: self.id.to_string(),
            version: self.version.to_string(),
            label: self.label.clone(),
            description: self.description.clone(),
            flow_id: self.flow_id.to_string(),
            transport: self.transport.to_string(),
            tags,
            device_id: self.device_id.to_string(),
            manifest_href: self.manifest_href.clone(),
        }
    }
}
