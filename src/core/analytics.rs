use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSessionRecord {
    pub session_id: String,
    pub instance_name: String,
    pub mc_version: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_seconds: u64,
    pub estimated_avg_fps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalAnalyticsArchive {
    pub total_playtime_hours: f64,
    pub total_sessions: u32,
    pub sessions: Vec<GameSessionRecord>,
}

pub struct AnalyticsManager {
    archive_path: PathBuf,
}

impl AnalyticsManager {
    pub fn new(minecraft_dir: &Path) -> Self {
        let path = minecraft_dir.join("analytics.json");
        Self { archive_path: path }
    }

    pub fn load_archive(&self) -> LocalAnalyticsArchive {
        if self.archive_path.exists() {
            if let Ok(content) = fs::read_to_string(&self.archive_path) {
                if let Ok(archive) = serde_json::from_str::<LocalAnalyticsArchive>(&content) {
                    return archive;
                }
            }
        }
        LocalAnalyticsArchive::default()
    }

    pub fn record_session(&self, record: GameSessionRecord) -> std::io::Result<()> {
        let mut archive = self.load_archive();
        archive.total_sessions += 1;
        archive.total_playtime_hours += (record.duration_seconds as f64) / 3600.0;
        archive.sessions.push(record);

        // Keep last 100 sessions
        if archive.sessions.len() > 100 {
            archive.sessions.remove(0);
        }

        let json = serde_json::to_string_pretty(&archive)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(&self.archive_path, json)
    }
}
