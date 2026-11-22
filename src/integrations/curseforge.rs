use reqwest::Client;
use serde::{Deserialize, Serialize};

pub const CURSEFORGE_API_URL: &str = "https://api.curseforge.com/v1";
pub const DEFAULT_CURSEFORGE_KEY: &str =
    "$2a$10$bL4bIL5pUWqfcO7KQtnMRehkWyMmRBuBz6GST5A8fYVNjUs3KOlRi";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurseForgeMod {
    pub id: u32,
    pub name: String,
    pub summary: String,
    pub download_count: Option<f64>,
    pub latest_files: Option<Vec<CurseForgeFile>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurseForgeFile {
    pub id: u32,
    pub display_name: String,
    pub file_name: String,
    pub download_url: Option<String>,
    pub game_versions: Vec<String>,
}

#[derive(Clone)]
pub struct CurseForgeClient {
    client: Client,
    api_key: String,
}

impl CurseForgeClient {
    pub fn new(api_key: Option<String>) -> Self {
        let key = api_key
            .filter(|k| !k.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_CURSEFORGE_KEY.to_string());

        Self {
            client: Client::builder()
                .user_agent("AD-Launcher-Linux/1.0.0 (Debian; Rust)")
                .build()
                .unwrap_or_default(),
            api_key: key,
        }
    }

    pub async fn search_mods(
        &self,
        query: &str,
        game_version: Option<&str>,
        loader_type: Option<u32>, // 0=any, 1=forge, 4=fabric, 5=quilt, 6=neoforge
        page_size: usize,
    ) -> Result<Vec<CurseForgeMod>, String> {
        let mut req = self
            .client
            .get(format!("{}/mods/search", CURSEFORGE_API_URL))
            .header("x-api-key", &self.api_key)
            .query(&[
                ("gameId", "432"),
                ("classId", "6"), // 6 is Mods
                ("searchFilter", query),
                ("pageSize", &page_size.to_string()),
            ]);

        if let Some(gv) = game_version {
            if !gv.is_empty() {
                req = req.query(&[("gameVersion", gv)]);
            }
        }

        if let Some(lt) = loader_type {
            if lt > 0 {
                req = req.query(&[("modLoaderType", &lt.to_string())]);
            }
        }

        let resp = req
            .send()
            .await
            .map_err(|e| format!("CurseForge search error: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("CurseForge returned status {}", resp.status()));
        }

        #[derive(Deserialize)]
        struct Response {
            data: Vec<CurseForgeRawMod>,
        }

        #[derive(Deserialize)]
        struct CurseForgeRawMod {
            id: u32,
            name: String,
            summary: String,
            #[serde(rename = "downloadCount")]
            download_count: Option<f64>,
            #[serde(rename = "latestFiles")]
            latest_files: Option<Vec<CurseForgeRawFile>>,
        }

        #[derive(Deserialize)]
        struct CurseForgeRawFile {
            id: u32,
            #[serde(rename = "displayName")]
            display_name: String,
            #[serde(rename = "fileName")]
            file_name: String,
            #[serde(rename = "downloadUrl")]
            download_url: Option<String>,
            #[serde(rename = "gameVersions")]
            game_versions: Vec<String>,
        }

        let res: Response = resp
            .json()
            .await
            .map_err(|e| format!("JSON parse error: {}", e))?;
        let mods = res
            .data
            .into_iter()
            .map(|m| CurseForgeMod {
                id: m.id,
                name: m.name,
                summary: m.summary,
                download_count: m.download_count,
                latest_files: m.latest_files.map(|files| {
                    files
                        .into_iter()
                        .map(|f| CurseForgeFile {
                            id: f.id,
                            display_name: f.display_name,
                            file_name: f.file_name,
                            download_url: f.download_url,
                            game_versions: f.game_versions,
                        })
                        .collect()
                }),
            })
            .collect();

        Ok(mods)
    }

    /// Fetches the primary download file for a mod
    pub async fn get_mod_download(&self, mod_id: u32) -> Result<(String, String), String> {
        let url = format!("{}/mods/{}/files", CURSEFORGE_API_URL, mod_id);
        let resp = self
            .client
            .get(&url)
            .header("x-api-key", &self.api_key)
            .query(&[("pageSize", "1")])
            .send()
            .await
            .map_err(|e| format!("Failed to fetch CurseForge files: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("CurseForge files error: {}", resp.status()));
        }

        #[derive(Deserialize)]
        struct FilesResponse {
            data: Vec<FileItem>,
        }

        #[derive(Deserialize)]
        struct FileItem {
            #[serde(rename = "downloadUrl")]
            download_url: Option<String>,
            #[serde(rename = "fileName")]
            file_name: String,
        }

        let res: FilesResponse = resp.json().await.map_err(|e| e.to_string())?;
        let file =
            res.data.into_iter().next().ok_or_else(|| {
                "No downloadable files found for this mod on CurseForge".to_string()
            })?;

        let dl_url = file.download_url.ok_or_else(|| {
            format!(
                "Direct download is restricted by author for {}",
                file.file_name
            )
        })?;

        Ok((dl_url, file.file_name))
    }
}
