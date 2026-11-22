use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const MODRINTH_API_URL: &str = "https://api.modrinth.com/v2";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthProject {
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub author: String,
    pub icon_url: Option<String>,
    pub downloads: u64,
    pub follows: u64,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthVersion {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub version_number: String,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub files: Vec<ModrinthFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthFile {
    pub hashes: ModrinthHashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModrinthHashes {
    pub sha1: String,
    pub sha512: Option<String>,
}

pub struct ModrinthClient {
    client: Client,
}

impl ModrinthClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("NUX-Launcher-Linux/1.0.0 (Debian; Rust)")
                .build()
                .unwrap_or_default(),
        }
    }

    /// Searches for Fabric mods for Minecraft 1.20.1
    pub async fn search_mods(
        &self,
        query: &str,
        game_version: &str,
        loader: &str,
        limit: usize,
    ) -> Result<Vec<ModrinthProject>, String> {
        let facets = format!(
            "[[\"project_type:mod\"],[\"versions:{}\"],[\"categories:{}\"]]",
            game_version, loader
        );

        let url = format!("{}/search", MODRINTH_API_URL);
        let resp = self
            .client
            .get(&url)
            .query(&[
                ("query", query),
                ("facets", &facets),
                ("limit", &limit.to_string()),
            ])
            .send()
            .await
            .map_err(|e| format!("Modrinth search error: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Modrinth API error: {}", resp.status()));
        }

        #[derive(Deserialize)]
        struct SearchResponse {
            hits: Vec<ModrinthProject>,
        }

        let result: SearchResponse = resp.json().await.map_err(|e| e.to_string())?;
        Ok(result.hits)
    }

    /// Fetches the latest compatible version of a mod for 1.20.1 Fabric
    pub async fn get_latest_version(
        &self,
        project_id_or_slug: &str,
        game_version: &str,
        loader: &str,
    ) -> Result<ModrinthVersion, String> {
        let url = format!(
            "{}/project/{}/version",
            MODRINTH_API_URL, project_id_or_slug
        );
        let resp = self
            .client
            .get(&url)
            .query(&[
                ("game_versions", format!("[\"{}\"]", game_version)),
                ("loaders", format!("[\"{}\"]", loader)),
            ])
            .send()
            .await
            .map_err(|e| format!("Modrinth version fetch failed: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Modrinth API status: {}", resp.status()));
        }

        let versions: Vec<ModrinthVersion> = resp.json().await.map_err(|e| e.to_string())?;
        versions
            .into_iter()
            .next()
            .ok_or_else(|| format!("No compatible version found for {}", project_id_or_slug))
    }

    /// Downloads the primary jar file of a mod directly to destination directory
    pub async fn download_mod_file(
        &self,
        version: &ModrinthVersion,
        dest_dir: &Path,
    ) -> Result<std::path::PathBuf, String> {
        let file = version
            .files
            .iter()
            .find(|f| f.primary)
            .or_else(|| version.files.first())
            .ok_or_else(|| "No files found in mod version".to_string())?;

        let dest_path = dest_dir.join(&file.filename);
        let resp = self
            .client
            .get(&file.url)
            .send()
            .await
            .map_err(|e| format!("Failed to download mod: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Download failed with status {}", resp.status()));
        }

        let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
        std::fs::write(&dest_path, bytes).map_err(|e| format!("Failed to write mod jar: {}", e))?;

        Ok(dest_path)
    }
}
