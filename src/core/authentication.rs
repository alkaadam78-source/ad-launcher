use crate::utils::fs::{ensure_dir, launcher_config_dir};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use uuid::Uuid;

// Standard Microsoft OAuth Client ID used by Minecraft launchers
pub const MS_CLIENT_ID: &str = "00000000402b5328";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountProfile {
    pub username: String,
    pub uuid: String,
    pub is_microsoft: bool,
    pub skin_url: Option<String>,
    #[serde(skip_serializing)]
    pub access_token: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceCodeResponse {
    pub user_code: String,
    pub device_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct TokenResponse {
    access_token: Option<String>,
    refresh_token: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct MinecraftProfileResponse {
    id: String,
    name: String,
    skins: Option<Vec<MinecraftSkin>>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct MinecraftSkin {
    id: String,
    state: String,
    url: String,
}

pub struct AuthManager {
    client: Client,
    auth_file: PathBuf,
}

impl AuthManager {
    pub fn new() -> Self {
        let auth_file = launcher_config_dir().join("auth.json");
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("NUX-Launcher-Linux/1.0.0 (Debian; Rust)")
            .build()
            .unwrap_or_default();

        Self { client, auth_file }
    }

    pub fn load_active_account(&self) -> Option<AccountProfile> {
        if self.auth_file.exists() {
            if let Ok(content) = fs::read_to_string(&self.auth_file) {
                if let Ok(profile) = serde_json::from_str::<AccountProfile>(&content) {
                    return Some(profile);
                }
            }
        }
        None
    }

    pub fn save_active_account(&self, profile: &AccountProfile) -> std::io::Result<()> {
        if let Some(p) = self.auth_file.parent() {
            ensure_dir(p)?;
        }
        let json = serde_json::to_string_pretty(profile)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(&self.auth_file, json)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&self.auth_file, fs::Permissions::from_mode(0o600));
        }

        Ok(())
    }

    pub fn logout(&self) -> std::io::Result<()> {
        if self.auth_file.exists() {
            fs::remove_file(&self.auth_file)?;
        }
        Ok(())
    }

    /// Sets up a local offline player profile
    pub fn create_offline_profile(&self, username: &str) -> AccountProfile {
        let clean_name = if username.trim().is_empty() {
            "Player".to_string()
        } else {
            username.trim().to_string()
        };

        // Deterministic offline UUID based on "OfflinePlayer:<name>"
        let offline_uuid = Uuid::new_v5(
            &Uuid::NAMESPACE_DNS,
            format!("OfflinePlayer:{}", clean_name).as_bytes(),
        );

        AccountProfile {
            username: clean_name,
            uuid: offline_uuid.simple().to_string(),
            is_microsoft: false,
            skin_url: None,
            access_token: "offline_token".to_string(),
            refresh_token: None,
        }
    }

    /// Sets up a designated Demo Mode profile with clearly labeled sample data
    pub fn create_demo_profile(&self) -> AccountProfile {
        AccountProfile {
            username: "Demo Explorer (Sample Data)".to_string(),
            uuid: "00000000-0000-0000-0000-000000000000".to_string(),
            is_microsoft: false,
            skin_url: None,
            access_token: "DEMO_SIMULATED_SESSION".to_string(),
            refresh_token: None,
        }
    }

    /// Step 1: Initiate Microsoft OAuth Device Code Flow
    pub async fn initiate_device_code(&self) -> Result<DeviceCodeResponse, String> {
        let params = [
            ("client_id", MS_CLIENT_ID),
            ("scope", "service::user.auth.xboxlive.com::MBI_SSL"),
        ];

        let resp = self
            .client
            .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode")
            .form(&params)
            .send()
            .await
            .map_err(|e| format!("Failed to initiate device code login: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!(
                "Device code request failed with status: {}",
                resp.status()
            ));
        }

        resp.json::<DeviceCodeResponse>()
            .await
            .map_err(|e| format!("Failed to parse device code response: {}", e))
    }

    /// Step 2: Poll for MSA Token
    pub async fn poll_device_code_token(
        &self,
        device_code: &str,
        interval_secs: u64,
        max_attempts: usize,
    ) -> Result<String, String> {
        let params = [
            ("client_id", MS_CLIENT_ID),
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ("device_code", device_code),
        ];

        let sleep_dur = Duration::from_secs(interval_secs.max(2));

        for _ in 0..max_attempts {
            tokio::time::sleep(sleep_dur).await;

            let resp = self
                .client
                .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
                .form(&params)
                .send()
                .await
                .map_err(|e| format!("Token poll failed: {}", e))?;

            if let Ok(token_resp) = resp.json::<TokenResponse>().await {
                if let Some(token) = token_resp.access_token {
                    return Ok(token);
                }
                if let Some(err) = token_resp.error {
                    if err != "authorization_pending" {
                        return Err(format!("Authorization error: {}", err));
                    }
                }
            }
        }

        Err("Device code authorization timed out".to_string())
    }

    /// Step 3: Complete Xbox Live -> XSTS -> Minecraft services authentication
    pub async fn complete_minecraft_login(
        &self,
        msa_token: &str,
    ) -> Result<AccountProfile, String> {
        // 1. Xbox Live authentication
        let xbl_body = serde_json::json!({
            "Properties": {
                "AuthMethod": "RPS",
                "SiteName": "user.auth.xboxlive.com",
                "RpsTicket": format!("d={}", msa_token)
            },
            "RelyingParty": "http://auth.xboxlive.com",
            "TokenType": "JWT"
        });

        let xbl_resp = self
            .client
            .post("https://user.auth.xboxlive.com/user/authenticate")
            .json(&xbl_body)
            .send()
            .await
            .map_err(|e| format!("XBL auth failed: {}", e))?;

        if !xbl_resp.status().is_success() {
            return Err("Xbox Live authentication failed".to_string());
        }

        let xbl_json: serde_json::Value = xbl_resp.json().await.map_err(|e| e.to_string())?;
        let xbl_token = xbl_json["Token"].as_str().ok_or("Missing XBL Token")?;
        let user_hash = xbl_json["DisplayClaims"]["xui"][0]["uhs"]
            .as_str()
            .ok_or("Missing user hash")?;

        // 2. XSTS Token authentication
        let xsts_body = serde_json::json!({
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [xbl_token]
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT"
        });

        let xsts_resp = self
            .client
            .post("https://xsts.auth.xboxlive.com/xsts/authorize")
            .json(&xsts_body)
            .send()
            .await
            .map_err(|e| format!("XSTS auth failed: {}", e))?;

        if !xsts_resp.status().is_success() {
            return Err(
                "XSTS authentication failed. Account might not have Xbox profile.".to_string(),
            );
        }

        let xsts_json: serde_json::Value = xsts_resp.json().await.map_err(|e| e.to_string())?;
        let xsts_token = xsts_json["Token"].as_str().ok_or("Missing XSTS Token")?;

        // 3. Minecraft Service Login
        let mc_login_body = serde_json::json!({
            "identityToken": format!("XBL3.0 x={};{}", user_hash, xsts_token)
        });

        let mc_resp = self
            .client
            .post("https://api.minecraftservices.com/authentication/login_with_xbox")
            .json(&mc_login_body)
            .send()
            .await
            .map_err(|e| format!("Minecraft login endpoint failed: {}", e))?;

        if !mc_resp.status().is_success() {
            return Err("Failed to authenticate with Minecraft Services".to_string());
        }

        let mc_json: serde_json::Value = mc_resp.json().await.map_err(|e| e.to_string())?;
        let mc_token = mc_json["access_token"]
            .as_str()
            .ok_or("Missing Minecraft access token")?;

        // 4. Verify Minecraft Ownership / Entitlements
        let store_resp = self
            .client
            .get("https://api.minecraftservices.com/entitlements/mcstore")
            .bearer_auth(mc_token)
            .send()
            .await
            .map_err(|e| format!("Failed to verify game ownership: {}", e))?;

        if !store_resp.status().is_success() {
            return Err("Could not verify Minecraft game ownership.".to_string());
        }

        // 5. Fetch Profile
        let profile_resp = self
            .client
            .get("https://api.minecraftservices.com/minecraft/profile")
            .bearer_auth(mc_token)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch Minecraft profile: {}", e))?;

        if !profile_resp.status().is_success() {
            return Err(
                "Account does not own Minecraft Java Edition or has no profile created."
                    .to_string(),
            );
        }

        let profile_data: MinecraftProfileResponse =
            profile_resp.json().await.map_err(|e| e.to_string())?;
        let skin_url = profile_data
            .skins
            .and_then(|s| s.into_iter().find(|sk| sk.state == "ACTIVE"))
            .map(|s| s.url);

        let profile = AccountProfile {
            username: profile_data.name,
            uuid: profile_data.id,
            is_microsoft: true,
            skin_url,
            access_token: mc_token.to_string(),
            refresh_token: None,
        };

        let _ = self.save_active_account(&profile);
        Ok(profile)
    }
}
