use std::collections::BTreeMap;

use nmos_rs_schema::is_04;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    resource::{Format, Source},
    tai::TaiTime,
    version::{APIVersion, is_04::V1_0},
};

pub struct FlowBuilder {
    label: Option<String>,
    description: Option<String>,
    format: Format,
    tags: BTreeMap<String, Vec<String>>,
    source_id: Uuid,
    parents: Vec<Uuid>,
}

impl FlowBuilder {
    pub fn new(source: &Source) -> FlowBuilder {
        FlowBuilder {
            label: None,
            description: None,
            format: source.format,
            tags: Default::default(),
            source_id: source.id,
            parents: Vec::new(),
        }
    }

    pub fn label<S: Into<String>>(mut self, label: S) -> FlowBuilder {
        self.label = Some(label.into());
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> FlowBuilder {
        self.description = Some(description.into());
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
    pub fn builder(source: &Source) -> FlowBuilder {
        FlowBuilder::new(source)
    }

    pub fn to_json(&self, api: &APIVersion) -> FlowJson {
        match *api {
            V1_0 => {
                // Tags
                let tags = self
                    .tags
                    .iter()
                    .fold(BTreeMap::new(), |mut map, (key, array)| {
                        let value = serde_json::Value::from(array.clone());
                        map.insert(key.clone(), value);
                        map
                    });

                let parents = self.parents.iter().map(|p| p.to_string()).collect();

                FlowJson::V1_0(is_04::v1_0_x::Flow {
                    id: self.id.to_string(),
                    version: self.version.to_string(),
                    label: self.label.clone(),
                    description: self.description.clone(),
                    format: self.format.to_string(),
                    tags,
                    source_id: self.source_id.to_string(),
                    parents,
                })
            }
            _ => panic!("Unsupported API"),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum FlowJson {
    V1_0(is_04::v1_0_x::Flow),
}
