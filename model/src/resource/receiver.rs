use std::collections::BTreeMap;

use nmos_schema::is_04;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    resource::{Device, Format, Transport},
    tai::TaiTime,
    version::{is_04::V1_0, APIVersion},
};

pub struct ReceiverBuilder {
    label: Option<String>,
    description: Option<String>,
    format: Format,
    tags: BTreeMap<String, Vec<String>>,
    device_id: Uuid,
    transport: Transport,
    subscription: Option<Uuid>,
}

impl ReceiverBuilder {
    pub fn new(device: &Device, format: Format, transport: Transport) -> ReceiverBuilder {
        ReceiverBuilder {
            label: None,
            description: None,
            format,
            tags: Default::default(),
            device_id: device.id,
            transport,
            subscription: None,
        }
    }

    pub fn label<S: Into<String>>(mut self, label: S) -> ReceiverBuilder {
        self.label = Some(label.into());
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> ReceiverBuilder {
        self.description = Some(description.into());
        self
    }

    pub fn build(self) -> Receiver {
        Receiver {
            id: Uuid::new_v4(),
            version: TaiTime::now(),
            label: self.label.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
            format: self.format,
            tags: self.tags,
            device_id: self.device_id,
            transport: self.transport,
            subscription: self.subscription,
        }
    }
}

#[derive(Debug)]
pub struct Receiver {
    pub id: Uuid,
    pub version: TaiTime,
    pub label: String,
    pub description: String,
    pub format: Format,
    pub tags: BTreeMap<String, Vec<String>>,
    pub device_id: Uuid,
    pub transport: Transport,
    pub subscription: Option<Uuid>,
}

impl Receiver {
    pub fn builder(device: &Device, format: Format, transport: Transport) -> ReceiverBuilder {
        ReceiverBuilder::new(device, format, transport)
    }

    pub fn to_json(&self, api: &APIVersion) -> ReceiverJson {
        match *api {
            V1_0 => {
                let tags = self
                    .tags
                    .iter()
                    .fold(BTreeMap::new(), |mut map, (key, array)| {
                        let value = serde_json::Value::from(array.clone());
                        map.insert(key.clone(), value);
                        map
                    });

                let subscription = is_04::v1_0_x::ReceiverSubscription {
                    sender_id: self.subscription.map(|s| s.to_string()),
                };

                ReceiverJson::V1_0(is_04::v1_0_x::Receiver {
                    id: self.id.to_string(),
                    version: self.version.to_string(),
                    label: self.label.clone(),
                    description: self.description.clone(),
                    format: self.format.to_string(),
                    caps: Default::default(),
                    tags,
                    device_id: self.device_id.to_string(),
                    transport: self.transport.to_string(),
                    subscription,
                })
            }
            _ => panic!("Unsupported API"),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ReceiverJson {
    V1_0(is_04::v1_0_x::Receiver),
}
