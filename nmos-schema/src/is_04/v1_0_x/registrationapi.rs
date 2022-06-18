use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum RegistrationApiBaseItem {
    Resource,
    Health,
}

pub type RegistrationApiBase = [RegistrationApiBaseItem; 2];
