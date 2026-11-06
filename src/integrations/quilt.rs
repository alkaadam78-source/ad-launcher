use reqwest::Client;
use serde::{Deserialize, Serialize};

pub const QUILT_META_URL: &str = "https://meta.quiltmc.org/v3";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuiltLoaderVersion {
    pub version: String,
    pub build: u32,
}

pub struct QuiltClient {
    client: Client,
}

impl QuiltClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_loaders_for_game_version(
        &self,
        game_version: &str,
    ) -> Result<Vec<QuiltLoaderVersion>, String> {
        let url = format!("{}/versions/loader/{}", QUILT_META_URL, game_version);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Quilt meta request failed: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Quilt meta returned status {}", resp.status()));
        }

        #[derive(Deserialize)]
        struct QuiltEntry {
            loader: QuiltLoaderVersion,
        }

        let entries: Vec<QuiltEntry> = resp.json().await.map_err(|e| e.to_string())?;
        Ok(entries.into_iter().map(|e| e.loader).collect())
    }
}
