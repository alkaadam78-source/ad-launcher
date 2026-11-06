use reqwest::Client;
use serde::{Deserialize, Serialize};

pub const FABRIC_META_URL: &str = "https://meta.fabricmc.net/v2";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricLoaderVersion {
    pub separator: Option<String>,
    pub build: Option<u32>,
    pub maven: Option<String>,
    pub version: String,
    pub stable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricGameVersion {
    pub version: String,
    pub stable: bool,
}

pub struct FabricClient {
    client: Client,
}

impl FabricClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("NUX-Launcher-Linux/1.0.0 (Debian; Rust)")
                .build()
                .unwrap_or_default(),
        }
    }

    /// Fetches all compatible Fabric loader versions for a specific Minecraft version
    pub async fn get_loaders_for_game_version(
        &self,
        game_version: &str,
    ) -> Result<Vec<FabricLoaderVersion>, String> {
        let url = format!("{}/versions/loader/{}", FABRIC_META_URL, game_version);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch Fabric loader versions: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Fabric meta returned status: {}", resp.status()));
        }

        #[derive(Deserialize)]
        struct LoaderEntry {
            loader: FabricLoaderVersion,
        }

        let entries: Vec<LoaderEntry> = resp.json().await.map_err(|e| e.to_string())?;
        Ok(entries.into_iter().map(|e| e.loader).collect())
    }

    /// Fetches the latest stable Fabric loader version for 1.20.1
    pub async fn get_latest_stable_loader(&self, game_version: &str) -> Result<String, String> {
        let loaders = self.get_loaders_for_game_version(game_version).await?;
        loaders
            .into_iter()
            .find(|l| l.stable)
            .map(|l| l.version)
            .ok_or_else(|| "No stable Fabric loader found".to_string())
    }
}
