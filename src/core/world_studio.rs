use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldInfo {
    pub folder_name: String,
    pub display_name: String,
    pub size_mb: f64,
    pub last_played_str: String,
    pub path: PathBuf,
}

pub struct WorldStudio;

impl WorldStudio {
    /// Scans ~/.minecraft/saves or instance saves directory for world directories
    pub fn discover_worlds(saves_dir: &Path) -> Vec<WorldInfo> {
        let mut worlds = Vec::new();
        if !saves_dir.exists() || !saves_dir.is_dir() {
            return worlds;
        }

        if let Ok(entries) = fs::read_dir(saves_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let folder_name = entry.file_name().to_string_lossy().to_string();
                    let size_mb = Self::calculate_dir_size_mb(&path);
                    let last_played = path
                        .metadata()
                        .and_then(|m| m.modified())
                        .map(|t| {
                            let datetime: chrono::DateTime<chrono::Local> = t.into();
                            datetime.format("%Y-%m-%d %H:%M").to_string()
                        })
                        .unwrap_or_else(|_| "Unknown".to_string());

                    worlds.push(WorldInfo {
                        folder_name: folder_name.clone(),
                        display_name: folder_name,
                        size_mb,
                        last_played_str: last_played,
                        path,
                    });
                }
            }
        }

        worlds.sort_by(|a, b| b.last_played_str.cmp(&a.last_played_str));
        worlds
    }

    fn calculate_dir_size_mb(dir: &Path) -> f64 {
        let mut total_bytes = 0u64;
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() {
                    if let Ok(m) = fs::metadata(&p) {
                        total_bytes += m.len();
                    }
                } else if p.is_dir() {
                    total_bytes += (Self::calculate_dir_size_mb(&p) * 1024.0 * 1024.0) as u64;
                }
            }
        }
        (total_bytes as f64) / (1024.0 * 1024.0)
    }
}
