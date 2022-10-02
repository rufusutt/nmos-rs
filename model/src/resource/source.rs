use std::collections::BTreeMap;

use nmos_schema::is_04;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    resource::{Device, Format},
    version::{is_04::V1_0, APIVersion},
};

use super::{ResourceCore, ResourceCoreBuilder};

#[must_use]
pub struct SourceBuilder {
    core: ResourceCoreBuilder,
    format: Format,
    device_id: Uuid,
    parents: Vec<Uuid>,
}

impl SourceBuilder {
    pub fn new<S: Into<String>>(label: S, device: &Device, format: Format) -> Self {
        SourceBuilder {
            core: ResourceCoreBuilder::new(label),
            format,
            device_id: device.core.id,
            parents: Vec::new(),
        }
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.core = self.core.description(description);
        self
    }

    #[must_use]
    pub fn build(self) -> Source {
        Source {
            core: self.core.build(),
            format: self.format,
            device_id: self.device_id,
            parents: self.parents,
        }
    }
}

#[derive(Debug)]
pub struct Source {
    pub core: ResourceCore,
    pub format: Format,
    pub device_id: Uuid,
    pub parents: Vec<Uuid>,
}

impl Source {
    pub fn builder<S: Into<String>>(label: S, device: &Device, format: Format) -> SourceBuilder {
        SourceBuilder::new(label, device, format)
    }

    #[must_use]
    pub fn to_json(&self, api: &APIVersion) -> SourceJson {
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

                let parents = self.parents.iter().map(ToString::to_string).collect();

                SourceJson::V1_0(is_04::v1_0_x::Source {
                    id: self.core.id.to_string(),
                    version: self.core.version.to_string(),
                    label: self.core.label.clone(),
                    description: self.core.description.clone(),
                    format: self.format.to_string(),
                    caps: BTreeMap::default(),
                    tags,
                    device_id: self.device_id.to_string(),
                    parents,
                })
            }
            _ => panic!("Unsupported API"),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum SourceJson {
    V1_0(is_04::v1_0_x::Source),
}
