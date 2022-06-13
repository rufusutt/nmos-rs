use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Flow {
    pub id: Uuid,
    pub version: String,
    pub label: String,
    pub description: String,
    pub format: String,
    pub tags: HashMap<String, Vec<String>>,
    pub source_id: Uuid,
    pub parents: Vec<Uuid>,
}
