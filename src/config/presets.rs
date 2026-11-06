use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationPreset {
    Balanced,
    LowResource,
    Custom,
}

impl OptimizationPreset {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Balanced => "BALANCED",
            Self::LowResource => "LOW RESOURCE",
            Self::Custom => "CUSTOM",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Balanced => "3 GB RAM, 6 Chunks, 120 FPS Limit, Sodium + Iris + Lithium (Recommended for i3/8GB)",
            Self::LowResource => "2 GB RAM, 4 Chunks, 90 FPS Limit, Shaders disabled, Maximum Battery/Low Heat",
            Self::Custom => "User configured parameters",
        }
    }

    pub fn default_ram_mb(&self) -> u32 {
        match self {
            Self::Balanced => 3072,
            Self::LowResource => 2048,
            Self::Custom => 3072,
        }
    }

    pub fn render_distance(&self) -> u32 {
        match self {
            Self::Balanced => 6,
            Self::LowResource => 4,
            Self::Custom => 6,
        }
    }

    pub fn simulation_distance(&self) -> u32 {
        match self {
            Self::Balanced => 4,
            Self::LowResource => 3,
            Self::Custom => 4,
        }
    }

    pub fn max_fps(&self) -> u32 {
        match self {
            Self::Balanced => 120,
            Self::LowResource => 90,
            Self::Custom => 120,
        }
    }

    pub fn recommended_jvm_flags(&self) -> &'static str {
        "-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200 -XX:+UnlockExperimentalVMOptions -XX:+DisableExplicitGC -XX:+AlwaysPreTouch -XX:G1NewSizePercent=30 -XX:G1MaxNewSizePercent=40 -XX:G1ReservePercent=20 -XX:G1HeapWastePercent=5 -XX:G1MixedGCCountTarget=4 -XX:InitiatingHeapOccupancyPercent=15 -XX:G1PeriodicGCInterval=60000"
    }
}
