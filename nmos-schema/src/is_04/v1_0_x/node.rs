use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeService {
    href: String,
    ty: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub id: Uuid,
    pub version: String,
    pub label: String,
    pub href: String,
    pub hostname: String,
    pub caps: Option<()>,
    pub services: Vec<NodeService>,
}
