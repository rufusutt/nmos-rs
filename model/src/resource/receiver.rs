use std::collections::BTreeMap;

use nmos_schema::is_04;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    resource::{Device, Format, Transport},
    version::{is_04::V1_0, APIVersion},
};

use super::{ResourceCore, ResourceCoreBuilder};

#[must_use]
pub struct ReceiverBuilder {
    core: ResourceCoreBuilder,
    format: Format,
    device_id: Uuid,
    transport: Transport,
    subscription: Option<Uuid>,
}

impl ReceiverBuilder {
    pub fn new<S: Into<String>>(
        label: S,
        device: &Device,
        format: Format,
        transport: Transport,
    ) -> Self {
        ReceiverBuilder {
            core: ResourceCoreBuilder::new(label),
            format,
            device_id: device.core.id,
            transport,
            subscription: None,
        }
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.core = self.core.description(description);
        self
    }

    pub fn tag<S, V>(mut self, key: S, values: V) -> Self
    where
        S: Into<String>,
        V: IntoIterator<Item = S>,
    {
        self.core = self.core.tag(key, values);
        self
    }

    #[must_use]
    pub fn build(self) -> Receiver {
        Receiver {
            core: self.core.build(),
            format: self.format,
            device_id: self.device_id,
            transport: self.transport,
            subscription: self.subscription,
        }
    }
}

#[derive(Debug)]
pub struct Receiver {
    pub core: ResourceCore,
    pub format: Format,
    pub device_id: Uuid,
    pub transport: Transport,
    pub subscription: Option<Uuid>,
}

impl Receiver {
    pub fn builder<S: Into<String>>(
        label: S,
        device: &Device,
        format: Format,
        transport: Transport,
    ) -> ReceiverBuilder {
        ReceiverBuilder::new(label, device, format, transport)
    }

    #[must_use]
    pub fn to_json(&self, api: &APIVersion) -> ReceiverJson {
        match *api {
            V1_0 => {
                let tags = self
                    .core
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
                    id: self.core.id.to_string(),
                    version: self.core.version.to_string(),
                    label: self.core.label.clone(),
                    description: self.core.description.clone(),
                    format: self.format.to_string(),
                    caps: BTreeMap::default(),
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
