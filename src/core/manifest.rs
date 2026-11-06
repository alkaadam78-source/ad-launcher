use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const OFFICIAL_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionManifest {
    pub latest: LatestVersion,
    pub versions: Vec<VersionEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestVersion {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionEntry {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    pub sha1: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionPackage {
    pub id: String,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    #[serde(rename = "minimumLauncherVersion")]
    pub minimum_launcher_version: Option<u32>,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    pub time: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub downloads: VersionDownloads,
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndexRef,
    #[serde(rename = "assets")]
    pub assets_id: String,
    pub libraries: Vec<Library>,
    pub arguments: Option<Arguments>,
    #[serde(rename = "minecraftArguments")]
    pub legacy_arguments: Option<String>,
    #[serde(rename = "javaVersion")]
    pub java_version: Option<JavaVersionRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaVersionRequirement {
    pub component: String,
    #[serde(rename = "majorVersion")]
    pub major_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDownloads {
    pub client: DownloadArtifact,
    pub client_mappings: Option<DownloadArtifact>,
    pub server: Option<DownloadArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadArtifact {
    pub sha1: String,
    pub size: u64,
    pub url: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetIndexRef {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    #[serde(rename = "totalSize")]
    pub total_size: u64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub name: String,
    pub downloads: Option<LibraryDownloads>,
    pub rules: Option<Vec<Rule>>,
    pub natives: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Option<DownloadArtifact>,
    pub classifiers: Option<HashMap<String, DownloadArtifact>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub action: String, // "allow" or "disallow"
    pub os: Option<OsRule>,
    pub features: Option<HashMap<String, bool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsRule {
    pub name: Option<String>,
    pub arch: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arguments {
    pub game: Vec<serde_json::Value>,
    pub jvm: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetIndex {
    pub objects: HashMap<String, AssetObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetObject {
    pub hash: String,
    pub size: u64,
}

impl Library {
    /// Determines whether this library should be included on Linux x86_64
    pub fn is_applicable_for_linux(&self) -> bool {
        if let Some(rules) = &self.rules {
            let mut allowed = false;
            for rule in rules {
                let matches_os = match &rule.os {
                    Some(os) => {
                        let name_matches = os.name.as_deref() == Some("linux");
                        let arch_matches = os.arch.as_deref().map(|a| a == "x86_64" || a == "x64").unwrap_or(true);
                        name_matches && arch_matches
                    }
                    None => true,
                };

                // Ignore special features like is_demo_user for client launch
                let matches_features = rule.features.is_none();

                if matches_os && matches_features {
                    if rule.action == "allow" {
                        allowed = true;
                    } else if rule.action == "disallow" {
                        allowed = false;
                    }
                }
            }
            allowed
        } else {
            // Default is allow if no rules are specified
            true
        }
    }

    /// Returns the local relative file path for library maven artifact
    pub fn artifact_path(&self) -> Option<String> {
        if let Some(downloads) = &self.downloads {
            if let Some(artifact) = &downloads.artifact {
                if let Some(path) = &artifact.path {
                    return Some(path.clone());
                }
            }
        }
        // Fallback: parse maven coordinate e.g. "com.mojang:patchy:1.1"
        self.maven_to_path()
    }

    /// Converts maven coordinate (group:artifact:version[:classifier]) to relative path
    pub fn maven_to_path(&self) -> Option<String> {
        let parts: Vec<&str> = self.name.split(':').collect();
        if parts.len() < 3 {
            return None;
        }
        let group = parts[0].replace('.', "/");
        let artifact = parts[1];
        let version = parts[2];
        let classifier = if parts.len() > 3 {
            format!("-{}", parts[3])
        } else {
            String::new()
        };

        Some(format!(
            "{}/{}/{}/{}-{}{}.jar",
            group, artifact, version, artifact, version, classifier
        ))
    }
}
