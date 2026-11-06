use reqwest::Client;
use serde::{Deserialize, Serialize};

pub const CURSEFORGE_API_URL: &str = "https://api.curseforge.com/v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurseForgeMod {
    pub id: u32,
    pub name: String,
    pub summary: String,
    pub download_count: f64,
}

pub struct CurseForgeClient {
    client: Client,
    api_key: Option<String>,
}

impl CurseForgeClient {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub fn is_configured(&self) -> bool {
        self.api_key.is_some()
    }

    pub async fn search_mods(&self, _query: &str) -> Result<Vec<CurseForgeMod>, String> {
        let key = match &self.api_key {
            Some(k) if !k.trim().is_empty() => k,
            _ => {
                return Err(
                    "CurseForge integration requires an x-api-key from Eternal API. Modrinth is recommended for open-source mods."
                        .to_string(),
                );
            }
        };

        // Query Curseforge endpoint if key is provided
        let resp = self
            .client
            .get(format!("{}/mods/search", CURSEFORGE_API_URL))
            .header("x-api-key", key)
            .send()
            .await
            .map_err(|e| format!("CurseForge request error: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("CurseForge returned status {}", resp.status()));
        }

        #[derive(Deserialize)]
        struct Response {
            data: Vec<CurseForgeMod>,
        }

        let res: Response = resp.json().await.map_err(|e| e.to_string())?;
        Ok(res.data)
    }
}
