use std::collections::BTreeMap;

use nmos_rs_schema::is_04;
use uuid::Uuid;

use crate::tai::TaiTime;

use super::{Device, Format, Resource};

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
    pub label: Option<String>,
    pub description: Option<String>,
    pub format: Format,
    pub tags: BTreeMap<String, Vec<String>>,
    pub device_id: Uuid,
    pub parents: Vec<Uuid>,
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

    pub fn label(mut self, label: String) -> SourceBuilder {
        self.label = Some(label);
        self
    }

    pub fn description(mut self, description: String) -> SourceBuilder {
        self.description = Some(description);
        self
    }

    pub fn build(self) -> Source {
        Source {
            id: Uuid::new_v4(),
            version: TaiTime::now(),
            label: self.label.unwrap_or(String::new()),
            description: self.description.unwrap_or(String::new()),
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
