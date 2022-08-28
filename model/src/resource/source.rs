use std::collections::BTreeMap;

use nmos_rs_schema::is_04;
use uuid::Uuid;

use super::{Device, Format, Resource};
use crate::tai::TaiTime;

#[derive(Debug)]
pub struct Source {
    pub id: Uuid,
    pub version: TaiTime,
    pub label: String,
    pub description: String,
    pub format: Format,
    pub tags: BTreeMap<String, Vec<String>>,
    pub device_id: Uuid,
    pub parents: Vec<Uuid>,
}

impl Source {
    pub fn builder(device: &Device, format: Format) -> SourceBuilder {
        SourceBuilder::new(device, format)
    }
}

pub struct SourceBuilder {
    label: Option<String>,
    description: Option<String>,
    format: Format,
    tags: BTreeMap<String, Vec<String>>,
    device_id: Uuid,
    parents: Vec<Uuid>,
}

impl SourceBuilder {
    pub fn new(device: &Device, format: Format) -> SourceBuilder {
        SourceBuilder {
            label: None,
            description: None,
            format,
            tags: Default::default(),
            device_id: device.id,
            parents: Vec::new(),
        }
    }

    pub fn label<S: Into<String>>(mut self, label: S) -> SourceBuilder {
        self.label = Some(label.into());
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> SourceBuilder {
        self.description = Some(description.into());
        self
    }

    pub fn build(self) -> Source {
        Source {
            id: Uuid::new_v4(),
            version: TaiTime::now(),
            label: self.label.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
            format: self.format,
            tags: self.tags,
            device_id: self.device_id,
            parents: self.parents,
        }
    }
}

impl Resource for Source {
    type JsonType = is_04::v1_0_x::SourceJson;

    fn to_json(&self) -> Self::JsonType {
        let tags = self
            .tags
            .iter()
            .fold(BTreeMap::new(), |mut map, (key, array)| {
                let value = serde_json::Value::from(array.clone());
                map.insert(key.clone(), value);
                map
            });

        let parents = self.parents.iter().map(|p| p.to_string()).collect();

        is_04::v1_0_x::SourceJson {
            id: self.id.to_string(),
            version: self.version.to_string(),
            label: self.label.clone(),
            description: self.description.clone(),
            format: self.format.to_string(),
            caps: Default::default(),
            tags,
            device_id: self.device_id.to_string(),
            parents,
        }
    }
}
