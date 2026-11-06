use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FpsOptimizationMode {
    AdamHyperPotato,
    AdamShaderSmooth,
    BalancedEdutainment,
    BatterySaver,
}

impl FpsOptimizationMode {
    pub fn title(&self) -> &'static str {
        match self {
            Self::AdamHyperPotato => "🚀 ADAM HYPER POTATO (200+ FPS)",
            Self::AdamShaderSmooth => "✨ ADAM SHADER SMOOTH (100-140 FPS)",
            Self::BalancedEdutainment => "🎮 BALANCED EDUTAINMENT (80-120 FPS)",
            Self::BatterySaver => "🔋 BATTERY SAVER / LOW HEAT (60 FPS)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::AdamHyperPotato => "Pure raw speed. 4 chunks, minimal particles, shadows off, uncapped 260 FPS, Mesa glthread.",
            Self::AdamShaderSmooth => "Cinematic potato visuals. 5 chunks, miniature/makeup shader active, tuned 8-chunk shadows, 144 FPS.",
            Self::BalancedEdutainment => "Solid all-rounder. 6 chunks, fast graphics, all entity animations enabled, 120 FPS.",
            Self::BatterySaver => "Quiet fan & cool temps. 4 chunks, 60 FPS lock, reduced memory allocation (2 GB), minimal CPU draw.",
        }
    }

    pub fn render_distance(&self) -> u32 {
        match self {
            Self::AdamHyperPotato => 4,
            Self::AdamShaderSmooth => 5,
            Self::BalancedEdutainment => 6,
            Self::BatterySaver => 4,
        }
    }

    pub fn simulation_distance(&self) -> u32 {
        match self {
            Self::AdamHyperPotato => 3,
            Self::AdamShaderSmooth => 4,
            Self::BalancedEdutainment => 4,
            Self::BatterySaver => 3,
        }
    }

    pub fn max_fps(&self) -> u32 {
        match self {
            Self::AdamHyperPotato => 260,
            Self::AdamShaderSmooth => 144,
            Self::BalancedEdutainment => 120,
            Self::BatterySaver => 60,
        }
    }

    pub fn ram_mb(&self) -> u32 {
        match self {
            Self::AdamHyperPotato => 2560,
            Self::AdamShaderSmooth => 3072,
            Self::BalancedEdutainment => 3072,
            Self::BatterySaver => 2048,
        }
    }
}

pub struct FpsBooster {
    minecraft_dir: PathBuf,
}

impl FpsBooster {
    pub fn new(minecraft_dir: &Path) -> Self {
        Self {
            minecraft_dir: minecraft_dir.to_path_buf(),
        }
    }

    pub fn apply_mode(&self, mode: FpsOptimizationMode) -> Result<String, String> {
        let options_path = self.minecraft_dir.join("options.txt");
        let sodium_path = self.minecraft_dir.join("config").join("sodium-options.json");
        let iris_path = self.minecraft_dir.join("config").join("iris.properties");

        // 1. Update options.txt
        let mut options_content = if options_path.exists() {
            fs::read_to_string(&options_path).unwrap_or_default()
        } else {
            String::new()
        };

        Self::upsert_option(&mut options_content, "renderDistance", &mode.render_distance().to_string());
        Self::upsert_option(&mut options_content, "simulationDistance", &mode.simulation_distance().to_string());
        Self::upsert_option(&mut options_content, "maxFps", &mode.max_fps().to_string());
        Self::upsert_option(&mut options_content, "enableVsync", "false");
        Self::upsert_option(&mut options_content, "graphicsMode", "0");
        Self::upsert_option(&mut options_content, "renderClouds", "\"false\"");
        Self::upsert_option(&mut options_content, "particles", if mode == FpsOptimizationMode::AdamHyperPotato || mode == FpsOptimizationMode::BatterySaver { "2" } else { "1" });

        fs::write(&options_path, options_content)
            .map_err(|e| format!("Failed to update options.txt: {}", e))?;

        // 2. Update Sodium options for 2 cores / 4 threads
        fs::create_dir_all(self.minecraft_dir.join("config")).map_err(|e| e.to_string())?;
        let sodium_json = format!(
            r#"{{
  "quality": {{
    "weather_quality": "FAST",
    "leaves_quality": "FAST",
    "enable_vignette": false
  }},
  "advanced": {{
    "enable_memory_tracing": false,
    "use_advanced_staging_buffers": true,
    "cpu_render_ahead_limit": 1
  }},
  "performance": {{
    "chunk_builder_threads": 2,
    "always_defer_chunk_updates_v2": true,
    "animate_only_visible_textures": true,
    "use_entity_culling": true,
    "use_fog_occlusion": true,
    "use_block_face_culling": true,
    "use_no_error_g_l_context": true
  }},
  "notifications": {{
    "has_cleared_donation_button": true,
    "has_seen_donation_prompt": true
  }}
}}"#
        );
        let _ = fs::write(&sodium_path, sodium_json);

        // 3. Update Iris shaders configuration
        let enable_shader = mode == FpsOptimizationMode::AdamShaderSmooth;
        let shader_content = format!(
            "# Iris Shaders Configuration — Optimized by Ad Launcher\n\
             colorSpace=SRGB\n\
             disableUpdateMessage=true\n\
             enableDebugOptions=false\n\
             maxShadowRenderDistance={}\n\
             shaderPack=miniature-shader-2.19.zip\n\
             enableShaders={}\n",
            if mode == FpsOptimizationMode::AdamHyperPotato { 4 } else { 8 },
            enable_shader
        );
        let _ = fs::write(&iris_path, shader_content);

        Ok(format!(
            "Applied {}! (RAM: {} MB, Render: {} Chunks, Target: {} FPS)",
            mode.title(),
            mode.ram_mb(),
            mode.render_distance(),
            mode.max_fps()
        ))
    }

    fn upsert_option(content: &mut String, key: &str, value: &str) {
        let prefix = format!("{}:", key);
        let new_line = format!("{}{}\n", prefix, value);

        if let Some(pos) = content.find(&prefix) {
            if let Some(end) = content[pos..].find('\n') {
                content.replace_range(pos..pos + end + 1, &new_line);
            }
        } else {
            content.push_str(&new_line);
        }
    }
}
