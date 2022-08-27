use std::collections::BTreeMap;

use nmos_rs_schema::is_04;
use uuid::Uuid;

use super::{Format, Resource, Source};
use crate::tai::TaiTime;

#[derive(Debug)]
pub struct Flow {
    pub id: Uuid,
    pub version: TaiTime,
    pub label: String,
    pub description: String,
    pub format: Format,
    pub tags: BTreeMap<String, Vec<String>>,
    pub source_id: Uuid,
    pub parents: Vec<Uuid>,
}

impl Flow {
    pub fn builder(source: &Source, format: Format) -> FlowBuilder {
        FlowBuilder::new(source, format)
    }
}

pub struct FlowBuilder {
    pub label: Option<String>,
    pub description: Option<String>,
    pub format: Format,
    pub tags: BTreeMap<String, Vec<String>>,
    pub source_id: Uuid,
    pub parents: Vec<Uuid>,
}

impl FlowBuilder {
    pub fn new(source: &Source, format: Format) -> FlowBuilder {
        FlowBuilder {
            label: None,
            description: None,
            format,
            tags: Default::default(),
            source_id: source.id,
            parents: Vec::new(),
        }
    }

    pub fn label(mut self, label: String) -> FlowBuilder {
        self.label = Some(label);
        self
    }

    pub fn description(mut self, description: String) -> FlowBuilder {
        self.description = Some(description);
        self
    }

    pub fn build(self) -> Flow {
        Flow {
            id: Uuid::new_v4(),
            version: TaiTime::now(),
            label: self.label.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
            format: self.format,
            tags: self.tags,
            source_id: self.source_id,
            parents: self.parents,
        }
    }
}

impl Resource for Flow {
    type JsonType = is_04::v1_0_x::FlowJson;

    fn to_json(&self) -> Self::JsonType {
        // Flows
        let tags = self
            .tags
            .iter()
            .fold(BTreeMap::new(), |mut map, (key, array)| {
                let value = serde_json::Value::from(array.clone());
                map.insert(key.clone(), value);
                map
            });

        let parents = self.parents.iter().map(|p| p.to_string()).collect();

        is_04::v1_0_x::FlowJson {
            id: self.id.to_string(),
            version: self.version.to_string(),
            label: self.label.clone(),
            description: self.description.clone(),
            format: self.format.to_string(),
            tags,
            source_id: self.source_id.to_string(),
            parents,
        }
    }
}
