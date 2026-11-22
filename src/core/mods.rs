use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModInfo {
    pub file_name: String,
    pub path: PathBuf,
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub enabled: bool,
    pub is_performance_mod: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct FabricModMetadata {
    id: String,
    name: Option<String>,
    version: String,
    description: Option<String>,
    authors: Option<Vec<serde_json::Value>>,
}

pub struct ModManager {
    mods_dir: PathBuf,
}

impl ModManager {
    pub fn new<P: AsRef<Path>>(game_dir: P) -> Self {
        Self {
            mods_dir: game_dir.as_ref().join("mods"),
        }
    }

    pub fn mods_dir(&self) -> &Path {
        &self.mods_dir
    }

    pub fn list_mods(&self) -> Vec<ModInfo> {
        let mut mods = Vec::new();
        if !self.mods_dir.exists() {
            return mods;
        }

        if let Ok(entries) = fs::read_dir(&self.mods_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let file_name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let (enabled, is_mod_file) = if file_name.ends_with(".jar") {
                    (true, true)
                } else if file_name.ends_with(".jar.disabled") {
                    (false, true)
                } else {
                    (false, false)
                };

                if is_mod_file {
                    let mut info = Self::inspect_mod_jar(&path)
                        .unwrap_or_else(|| Self::fallback_mod_info(&path, enabled));
                    info.enabled = enabled;
                    info.is_performance_mod = Self::is_known_performance_mod(&info.id, &info.name);
                    mods.push(info);
                }
            }
        }

        mods.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        mods
    }

    pub fn toggle_mod(&self, mod_info: &ModInfo) -> std::io::Result<PathBuf> {
        let current_path = &mod_info.path;
        let new_path = if mod_info.enabled {
            // Disable: append .disabled
            current_path.with_extension("jar.disabled")
        } else {
            // Enable: remove .disabled
            let path_str = current_path.to_string_lossy();
            if path_str.ends_with(".disabled") {
                PathBuf::from(&path_str[..path_str.len() - 9])
            } else {
                current_path.with_extension("jar")
            }
        };

        fs::rename(current_path, &new_path)?;
        Ok(new_path)
    }

    pub fn delete_mod(&self, mod_info: &ModInfo) -> std::io::Result<()> {
        if mod_info.path.exists() {
            fs::remove_file(&mod_info.path)?;
        }
        Ok(())
    }

    fn inspect_mod_jar(path: &Path) -> Option<ModInfo> {
        let file = File::open(path).ok()?;
        let mut archive = ZipArchive::new(file).ok()?;

        if let Ok(mut meta_file) = archive.by_name("fabric.mod.json") {
            let mut content = String::new();
            if meta_file.read_to_string(&mut content).is_ok() {
                if let Ok(meta) = serde_json::from_str::<FabricModMetadata>(&content) {
                    let authors = meta
                        .authors
                        .map(|auths| {
                            auths
                                .into_iter()
                                .map(|v| match v {
                                    serde_json::Value::String(s) => s,
                                    serde_json::Value::Object(o) => o
                                        .get("name")
                                        .and_then(|n| n.as_str())
                                        .unwrap_or("Unknown")
                                        .to_string(),
                                    _ => "Unknown".to_string(),
                                })
                                .collect()
                        })
                        .unwrap_or_default();

                    let name = meta.name.unwrap_or_else(|| meta.id.clone());
                    let desc = meta.description.unwrap_or_default();

                    return Some(ModInfo {
                        file_name: path.file_name()?.to_string_lossy().to_string(),
                        path: path.to_path_buf(),
                        id: meta.id,
                        name,
                        version: meta.version,
                        description: desc,
                        authors,
                        enabled: true,
                        is_performance_mod: false,
                    });
                }
            }
        }

        None
    }

    fn fallback_mod_info(path: &Path, enabled: bool) -> ModInfo {
        let raw_name = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown Mod".to_string());

        let clean_name = raw_name.trim_end_matches(".jar").replace(['-', '_'], " ");

        ModInfo {
            file_name: path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default(),
            path: path.to_path_buf(),
            id: raw_name.clone(),
            name: clean_name,
            version: "Unknown".to_string(),
            description: "No metadata found inside mod archive.".to_string(),
            authors: Vec::new(),
            enabled,
            is_performance_mod: false,
        }
    }

    pub fn is_known_performance_mod(id: &str, name: &str) -> bool {
        let check = format!("{} {}", id.to_lowercase(), name.to_lowercase());
        check.contains("sodium")
            || check.contains("iris")
            || check.contains("lithium")
            || check.contains("ferritecore")
            || check.contains("indium")
            || check.contains("krypton")
            || check.contains("modernfix")
            || check.contains("immediatelyfast")
            || check.contains("entityculling")
            || check.contains("badoptimizations")
            || check.contains("dynamic_fps")
    }

    /// Checks status of the big four performance mods for Fabric 1.20.1
    pub fn check_core_performance_mods(&self) -> PerformanceModStatus {
        let mods = self.list_mods();
        let mut status = PerformanceModStatus::default();

        for m in mods {
            if !m.enabled {
                continue;
            }
            let check = format!("{} {}", m.id.to_lowercase(), m.name.to_lowercase());
            if check.contains("sodium") && !check.contains("reeses") && !check.contains("extra") {
                status.has_sodium = true;
            }
            if check.contains("iris") {
                status.has_iris = true;
            }
            if check.contains("lithium") {
                status.has_lithium = true;
            }
            if check.contains("ferritecore") {
                status.has_ferritecore = true;
            }
        }

        status
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PerformanceModStatus {
    pub has_sodium: bool,
    pub has_iris: bool,
    pub has_lithium: bool,
    pub has_ferritecore: bool,
}

impl PerformanceModStatus {
    pub fn is_fully_optimized(&self) -> bool {
        self.has_sodium && self.has_iris && self.has_lithium && self.has_ferritecore
    }
}
