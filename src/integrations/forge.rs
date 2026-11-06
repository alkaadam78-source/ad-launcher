use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeVersion {
    pub version: String,
    pub mc_version: String,
    pub is_recommended: bool,
}

pub struct ForgeIntegration;

impl ForgeIntegration {
    pub fn is_available() -> bool {
        // Forge for 1.20.1 requires Java 17+ and heavy installer unpacking
        true
    }

    pub fn status_description() -> &'static str {
        "Forge 1.20.1: Supported via existing profile in .minecraft/versions or standard Forge installer."
    }
}
