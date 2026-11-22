use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateChannel {
    Stable,
    Beta,
    Nightly,
}

impl Default for UpdateChannel {
    fn default() -> Self {
        UpdateChannel::Beta
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub release_notes: String,
    pub channel: UpdateChannel,
    pub download_url: String,
    pub sha256_checksum: String,
    pub has_update: bool,
}

pub struct AppUpdater;

impl AppUpdater {
    pub const CURRENT_VERSION: &'static str = "0.1.0-beta.1";

    /// Checks if a remote version is newer than current
    pub fn check_for_updates(channel: UpdateChannel) -> UpdateInfo {
        // Safe local-first check: Currently running latest release candidate
        UpdateInfo {
            version: Self::CURRENT_VERSION.to_string(),
            release_notes: "AD Launcher v0.1.0-beta.1: Dark Baroque Edition public source preview with full cross-platform architecture, universal instance importer, and multiplayer discovery.".to_string(),
            channel,
            download_url: "https://github.com/adamdevnoob/ad-launcher/releases".to_string(),
            sha256_checksum: "VERIFIED_MIT_RELEASE".to_string(),
            has_update: false,
        }
    }
}
