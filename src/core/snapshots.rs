use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceSnapshot {
    pub id: String,
    pub name: String,
    pub timestamp: String,
    pub mods_count: usize,
    pub size_bytes: u64,
    pub snapshot_path: PathBuf,
}

pub struct SnapshotManager {
    snapshots_dir: PathBuf,
}

impl SnapshotManager {
    pub fn new(minecraft_dir: &Path) -> Self {
        let dir = minecraft_dir.join("snapshots");
        let _ = fs::create_dir_all(&dir);
        Self { snapshots_dir: dir }
    }

    pub fn create_snapshot(
        &self,
        instance_id: &str,
        instance_dir: &Path,
        snapshot_name: &str,
    ) -> std::io::Result<InstanceSnapshot> {
        let now = chrono::Local::now();
        let timestamp_str = now.format("%Y-%m-%d_%H%M%S").to_string();
        let snap_id = format!("{}_{}", instance_id, timestamp_str);
        let target_dir = self.snapshots_dir.join(&snap_id);
        fs::create_dir_all(&target_dir)?;

        // Copy mods
        let source_mods = instance_dir.join("mods");
        let mut mods_count = 0;
        let mut total_size = 0u64;

        if source_mods.exists() {
            let dest_mods = target_dir.join("mods");
            fs::create_dir_all(&dest_mods)?;
            if let Ok(entries) = fs::read_dir(&source_mods) {
                for entry in entries.flatten() {
                    if entry.path().extension().map_or(false, |e| e == "jar") {
                        let dest = dest_mods.join(entry.file_name());
                        if let Ok(meta) = fs::metadata(entry.path()) {
                            total_size += meta.len();
                        }
                        let _ = fs::copy(entry.path(), dest);
                        mods_count += 1;
                    }
                }
            }
        }

        // Copy options.txt
        let options_file = instance_dir.join("options.txt");
        if options_file.exists() {
            let _ = fs::copy(&options_file, target_dir.join("options.txt"));
        }

        // Save metadata
        let snap = InstanceSnapshot {
            id: snap_id,
            name: snapshot_name.to_string(),
            timestamp: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            mods_count,
            size_bytes: total_size,
            snapshot_path: target_dir.clone(),
        };

        let meta_json = serde_json::to_string_pretty(&snap)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(target_dir.join("snapshot.json"), meta_json)?;

        Ok(snap)
    }

    pub fn list_snapshots(&self, instance_id: &str) -> Vec<InstanceSnapshot> {
        let mut list = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.snapshots_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && path.file_name().map_or(false, |n| n.to_string_lossy().starts_with(instance_id)) {
                    let meta_file = path.join("snapshot.json");
                    if meta_file.exists() {
                        if let Ok(content) = fs::read_to_string(&meta_file) {
                            if let Ok(snap) = serde_json::from_str::<InstanceSnapshot>(&content) {
                                list.push(snap);
                            }
                        }
                    }
                }
            }
        }
        list.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        list
    }

    pub fn restore_snapshot(&self, snapshot: &InstanceSnapshot, target_instance_dir: &Path) -> std::io::Result<()> {
        let snap_mods = snapshot.snapshot_path.join("mods");
        let target_mods = target_instance_dir.join("mods");
        if snap_mods.exists() {
            fs::create_dir_all(&target_mods)?;
            // Clean target mods first
            if let Ok(entries) = fs::read_dir(&target_mods) {
                for entry in entries.flatten() {
                    let _ = fs::remove_file(entry.path());
                }
            }
            // Restore from snapshot
            if let Ok(entries) = fs::read_dir(&snap_mods) {
                for entry in entries.flatten() {
                    let dest = target_mods.join(entry.file_name());
                    let _ = fs::copy(entry.path(), dest);
                }
            }
        }
        Ok(())
    }
}
