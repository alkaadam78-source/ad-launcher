use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoForgeVersion {
    pub version: String,
    pub mc_version: String,
}

pub struct NeoForgeIntegration;

impl NeoForgeIntegration {
    pub fn is_available() -> bool {
        // NeoForge primary focus starts at 1.20.2+; 1.20.1 has early fork
        true
    }

    pub fn status_description() -> &'static str {
        "NeoForge: Compatible with 1.20.2+ (1.20.1 beta available; Fabric recommended for 1.20.1 performance)."
    }
}
