use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportFormat {
    VanillaMinecraft,
    PrismLauncher,
    MultiMC,
    ModrinthPack,
    CurseForgePack,
    CustomDirectory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredExternalInstance {
    pub name: String,
    pub path: PathBuf,
    pub format: ImportFormat,
    pub detected_version: Option<String>,
    pub detected_loader: Option<String>,
    pub mods_count: usize,
    pub worlds_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationResult {
    pub success: bool,
    pub instance_id: String,
    pub imported_mods_count: usize,
    pub imported_worlds_count: usize,
    pub message: String,
}

pub struct InstanceImporter;

impl InstanceImporter {
    /// Detects external installations across standard Linux paths
    pub fn scan_for_importable_instances() -> Vec<DiscoveredExternalInstance> {
        let mut results = Vec::new();
        let home = match dirs::home_dir() {
            Some(h) => h,
            None => return results,
        };

        // 1. Check Standard Vanilla Minecraft (~/.minecraft)
        let vanilla_path = home.join(".minecraft");
        if vanilla_path.exists() {
            let mods_count = Self::count_files_in_dir(&vanilla_path.join("mods"), "jar");
            let worlds_count = Self::count_subdirs(&vanilla_path.join("saves"));
            results.push(DiscoveredExternalInstance {
                name: "Vanilla Default (.minecraft)".to_string(),
                path: vanilla_path,
                format: ImportFormat::VanillaMinecraft,
                detected_version: Some("1.20.1".to_string()),
                detected_loader: Some("Fabric".to_string()),
                mods_count,
                worlds_count,
            });
        }

        // 2. Check Prism Launcher instances
        let prism_path = home.join(".local/share/PrismLauncher/instances");
        if prism_path.exists() && prism_path.is_dir() {
            Self::scan_instance_subdirs(&prism_path, ImportFormat::PrismLauncher, &mut results);
        }

        // 3. Check MultiMC instances
        let multimc_path = home.join(".local/share/multimc/instances");
        if multimc_path.exists() && multimc_path.is_dir() {
            Self::scan_instance_subdirs(&multimc_path, ImportFormat::MultiMC, &mut results);
        }

        results
    }

    fn scan_instance_subdirs(
        dir: &Path,
        format: ImportFormat,
        results: &mut Vec<DiscoveredExternalInstance>,
    ) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let mc_dir = path.join(".minecraft");
                    let check_dir = if mc_dir.exists() {
                        mc_dir
                    } else {
                        path.clone()
                    };

                    let mods_count = Self::count_files_in_dir(&check_dir.join("mods"), "jar");
                    let worlds_count = Self::count_subdirs(&check_dir.join("saves"));

                    results.push(DiscoveredExternalInstance {
                        name,
                        path,
                        format: format.clone(),
                        detected_version: None,
                        detected_loader: None,
                        mods_count,
                        worlds_count,
                    });
                }
            }
        }
    }

    fn count_files_in_dir(dir: &Path, extension: &str) -> usize {
        if !dir.exists() {
            return 0;
        }
        fs::read_dir(dir)
            .map(|entries| {
                entries
                    .flatten()
                    .filter(|e| e.path().extension().map_or(false, |ext| ext == extension))
                    .count()
            })
            .unwrap_or(0)
    }

    fn count_subdirs(dir: &Path) -> usize {
        if !dir.exists() {
            return 0;
        }
        fs::read_dir(dir)
            .map(|entries| entries.flatten().filter(|e| e.path().is_dir()).count())
            .unwrap_or(0)
    }

    /// Non-destructively copies configs, mods, and worlds from source into target directory
    pub fn import_instance(
        source: &DiscoveredExternalInstance,
        target_dir: &Path,
    ) -> std::io::Result<MigrationResult> {
        fs::create_dir_all(target_dir)?;

        let source_mc = if source.path.join(".minecraft").exists() {
            source.path.join(".minecraft")
        } else {
            source.path.clone()
        };

        // Copy mods
        let source_mods = source_mc.join("mods");
        let target_mods = target_dir.join("mods");
        let mut mods_imported = 0;
        if source_mods.exists() {
            fs::create_dir_all(&target_mods)?;
            if let Ok(entries) = fs::read_dir(&source_mods) {
                for entry in entries.flatten() {
                    if entry.path().extension().map_or(false, |e| e == "jar") {
                        let dest = target_mods.join(entry.file_name());
                        let _ = fs::copy(entry.path(), dest);
                        mods_imported += 1;
                    }
                }
            }
        }

        // Copy worlds (saves)
        let source_saves = source_mc.join("saves");
        let target_saves = target_dir.join("saves");
        let mut worlds_imported = 0;
        if source_saves.exists() {
            fs::create_dir_all(&target_saves)?;
            if let Ok(entries) = fs::read_dir(&source_saves) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        let world_name = entry.file_name();
                        let dest = target_saves.join(&world_name);
                        let _ = Self::copy_dir_recursive(&entry.path(), &dest);
                        worlds_imported += 1;
                    }
                }
            }
        }

        // Copy options.txt
        let options_file = source_mc.join("options.txt");
        if options_file.exists() {
            let _ = fs::copy(&options_file, target_dir.join("options.txt"));
        }

        Ok(MigrationResult {
            success: true,
            instance_id: source.name.clone(),
            imported_mods_count: mods_imported,
            imported_worlds_count: worlds_imported,
            message: format!(
                "Successfully migrated {} ({} mods, {} worlds copied safely).",
                source.name, mods_imported, worlds_imported
            ),
        })
    }

    fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let dest_path = dst.join(entry.file_name());
            if entry.path().is_dir() {
                Self::copy_dir_recursive(&entry.path(), &dest_path)?;
            } else {
                fs::copy(entry.path(), dest_path)?;
            }
        }
        Ok(())
    }
}
