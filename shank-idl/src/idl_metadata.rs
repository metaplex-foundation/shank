use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlMetadata {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub address: Option<String>,
    /// shank
    pub origin: String,
}
