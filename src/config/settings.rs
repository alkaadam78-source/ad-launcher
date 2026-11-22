use crate::config::presets::OptimizationPreset;
use crate::utils::fs::{default_minecraft_dir, ensure_dir, launcher_config_dir};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherSettings {
    pub minecraft_dir: PathBuf,
    pub java_path: Option<PathBuf>,
    pub preset: OptimizationPreset,
    pub ram_mb: u32,
    pub custom_jvm_args: String,
    pub render_distance: u32,
    pub simulation_distance: u32,
    pub max_fps: u32,
    pub selected_instance: String,
    pub active_profile_name: String,
    pub active_profile_uuid: String,
    pub is_offline: bool,
    pub auto_close_on_launch: bool,
    pub enable_performance_gc: bool,
    pub window_width: u32,
    pub window_height: u32,
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub first_launch: bool,
    #[serde(default)]
    pub demo_mode: bool,
}

impl Default for LauncherSettings {
    fn default() -> Self {
        let preset = OptimizationPreset::Balanced;
        Self {
            minecraft_dir: default_minecraft_dir(),
            java_path: None,
            preset,
            ram_mb: preset.default_ram_mb(),
            custom_jvm_args: String::new(),
            render_distance: preset.render_distance(),
            simulation_distance: preset.simulation_distance(),
            max_fps: preset.max_fps(),
            selected_instance: "default".to_string(),
            active_profile_name: "Player".to_string(),
            active_profile_uuid: String::new(),
            is_offline: true,
            auto_close_on_launch: false,
            enable_performance_gc: true,
            window_width: 1024,
            window_height: 640,
            language: "en".to_string(),
            first_launch: false,
            demo_mode: false,
        }
    }
}

impl LauncherSettings {
    pub fn config_path() -> PathBuf {
        launcher_config_dir().join("settings.json")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(settings) = serde_json::from_str::<LauncherSettings>(&content) {
                    return settings;
                }
            }
        }
        let default_settings = Self::default();
        let _ = default_settings.save();
        default_settings
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            ensure_dir(parent)?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(path, json)
    }

    pub fn apply_preset(&mut self, preset: OptimizationPreset) {
        self.preset = preset;
        if preset != OptimizationPreset::Custom {
            self.ram_mb = preset.default_ram_mb();
            self.render_distance = preset.render_distance();
            self.simulation_distance = preset.simulation_distance();
            self.max_fps = preset.max_fps();
        }
    }
}
