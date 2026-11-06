use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct StorageBreakdown {
    pub total_mb: f64,
    pub assets_mb: f64,
    pub assets_count: usize,
    pub libraries_mb: f64,
    pub libraries_count: usize,
    pub versions_mb: f64,
    pub mods_mb: f64,
    pub mods_count: usize,
    pub shaders_mb: f64,
    pub shaders_count: usize,
    pub logs_mb: f64,
    pub logs_count: usize,
    pub crash_reports_count: usize,
}

pub struct StorageIntelligence {
    minecraft_dir: PathBuf,
}

impl StorageIntelligence {
    pub fn new(minecraft_dir: &Path) -> Self {
        Self {
            minecraft_dir: minecraft_dir.to_path_buf(),
        }
    }

    pub fn analyze(&self) -> StorageBreakdown {
        let mut breakdown = StorageBreakdown::default();

        // 1. Assets
        let (assets_size, assets_count) = Self::dir_stats(&self.minecraft_dir.join("assets").join("objects"));
        breakdown.assets_mb = (assets_size as f64) / (1024.0 * 1024.0);
        breakdown.assets_count = assets_count;

        // 2. Libraries
        let (libs_size, libs_count) = Self::dir_stats(&self.minecraft_dir.join("libraries"));
        breakdown.libraries_mb = (libs_size as f64) / (1024.0 * 1024.0);
        breakdown.libraries_count = libs_count;

        // 3. Versions
        let (versions_size, _) = Self::dir_stats(&self.minecraft_dir.join("versions"));
        breakdown.versions_mb = (versions_size as f64) / (1024.0 * 1024.0);

        // 4. Mods
        let (mods_size, mods_count) = Self::dir_stats(&self.minecraft_dir.join("mods"));
        breakdown.mods_mb = (mods_size as f64) / (1024.0 * 1024.0);
        breakdown.mods_count = mods_count;

        // 5. Shaders
        let (shaders_size, shaders_count) = Self::dir_stats(&self.minecraft_dir.join("shaderpacks"));
        breakdown.shaders_mb = (shaders_size as f64) / (1024.0 * 1024.0);
        breakdown.shaders_count = shaders_count;

        // 6. Logs & Crash Reports
        let (logs_size, logs_count) = Self::dir_stats(&self.minecraft_dir.join("logs"));
        let (crash_size, crash_count) = Self::dir_stats(&self.minecraft_dir.join("crash-reports"));
        breakdown.logs_mb = ((logs_size + crash_size) as f64) / (1024.0 * 1024.0);
        breakdown.logs_count = logs_count;
        breakdown.crash_reports_count = crash_count;

        let (total_size, _) = Self::dir_stats(&self.minecraft_dir);
        breakdown.total_mb = (total_size as f64) / (1024.0 * 1024.0);

        breakdown
    }

    pub fn clean_logs_and_crashes(&self) -> Result<(usize, u64), String> {
        let mut cleaned_files = 0;
        let mut freed_bytes = 0;

        let logs_dir = self.minecraft_dir.join("logs");
        if let Ok(entries) = fs::read_dir(logs_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".log.gz") || (name.ends_with(".log") && name != "latest.log") {
                    if let Ok(meta) = entry.metadata() {
                        freed_bytes += meta.len();
                    }
                    if fs::remove_file(&path).is_ok() {
                        cleaned_files += 1;
                    }
                }
            }
        }

        let crash_dir = self.minecraft_dir.join("crash-reports");
        if let Ok(entries) = fs::read_dir(crash_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Ok(meta) = entry.metadata() {
                    freed_bytes += meta.len();
                }
                if fs::remove_file(&path).is_ok() {
                    cleaned_files += 1;
                }
            }
        }

        Ok((cleaned_files, freed_bytes))
    }

    fn dir_stats(dir: &Path) -> (u64, usize) {
        let mut total_size = 0;
        let mut file_count = 0;

        if !dir.exists() {
            return (0, 0);
        }

        let mut stack = vec![dir.to_path_buf()];
        while let Some(current) = stack.pop() {
            if let Ok(entries) = fs::read_dir(current) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Ok(meta) = entry.metadata() {
                            total_size += meta.len();
                            file_count += 1;
                        }
                    } else if path.is_dir() {
                        stack.push(path);
                    }
                }
            }
        }

        (total_size, file_count)
    }
}
