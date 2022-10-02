use std::collections::BTreeMap;

use nmos_schema::is_04;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    resource::{Format, Source},
    version::{is_04::V1_0, APIVersion},
};

use super::{ResourceCore, ResourceCoreBuilder};

#[must_use]
pub struct FlowBuilder {
    core: ResourceCoreBuilder,
    format: Format,
    source_id: Uuid,
    parents: Vec<Uuid>,
}

impl FlowBuilder {
    pub fn new<S: Into<String>>(label: S, source: &Source) -> Self {
        FlowBuilder {
            core: ResourceCoreBuilder::new(label),
            format: source.format,
            source_id: source.core.id,
            parents: Vec::new(),
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
    pub fn build(self) -> Flow {
        Flow {
            core: self.core.build(),
            format: self.format,
            source_id: self.source_id,
            parents: self.parents,
        }
    }
}

#[derive(Debug)]
pub struct Flow {
    pub core: ResourceCore,
    pub format: Format,
    pub source_id: Uuid,
    pub parents: Vec<Uuid>,
}

impl Flow {
    pub fn builder<S: Into<String>>(label: S, source: &Source) -> FlowBuilder {
        FlowBuilder::new(label, source)
    }

    #[must_use]
    pub fn to_json(&self, api: &APIVersion) -> FlowJson {
        match *api {
            V1_0 => {
                // Tags
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

                FlowJson::V1_0(is_04::v1_0_x::Flow {
                    id: self.core.id.to_string(),
                    version: self.core.version.to_string(),
                    label: self.core.label.clone(),
                    description: self.core.description.clone(),
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
