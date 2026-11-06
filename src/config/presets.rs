use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationPreset {
    Balanced,
    LowResource,
    HyperBoost,
    Custom,
}

impl OptimizationPreset {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Balanced => "BALANCED",
            Self::LowResource => "LOW RESOURCE",
            Self::HyperBoost => "HYPER BOOST (ADAM TURBO)",
            Self::Custom => "CUSTOM",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Balanced => "3 GB RAM, 6 Chunks, 120 FPS Limit, Sodium + Iris + Lithium (Recommended for i3/8GB)",
            Self::LowResource => "2 GB RAM, 4 Chunks, 90 FPS Limit, Shaders disabled, Maximum Battery/Low Heat",
            Self::HyperBoost => "3 GB RAM, 5 Chunks, 240 FPS Target, Mesa glthread, Vectorized Math, Aikar G1GC, Potato Shaders Tuned (Credit By AdamDevnoob)",
            Self::Custom => "User configured parameters",
        }
    }

    pub fn default_ram_mb(&self) -> u32 {
        match self {
            Self::Balanced => 3072,
            Self::LowResource => 2048,
            Self::HyperBoost => 3072,
            Self::Custom => 3072,
        }
    }

    pub fn render_distance(&self) -> u32 {
        match self {
            Self::Balanced => 6,
            Self::LowResource => 4,
            Self::HyperBoost => 5,
            Self::Custom => 6,
        }
    }

    pub fn simulation_distance(&self) -> u32 {
        match self {
            Self::Balanced => 4,
            Self::LowResource => 3,
            Self::HyperBoost => 4,
            Self::Custom => 4,
        }
    }

    pub fn max_fps(&self) -> u32 {
        match self {
            Self::Balanced => 120,
            Self::LowResource => 90,
            Self::HyperBoost => 240,
            Self::Custom => 120,
        }
    }

    pub fn recommended_jvm_flags(&self) -> &'static str {
        match self {
            Self::HyperBoost => "-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=15 -XX:+UnlockExperimentalVMOptions -XX:+AlwaysPreTouch -XX:G1NewSizePercent=28 -XX:G1MaxNewSizePercent=38 -XX:G1ReservePercent=15 -XX:G1HeapWastePercent=5 -XX:G1MixedGCCountTarget=4 -XX:InitiatingHeapOccupancyPercent=15 -XX:G1MixedGCLiveThresholdPercent=90 -XX:G1RSetUpdatingPauseTimePercent=5 -XX:SurvivorRatio=32 -XX:+PerfDisableSharedMem -XX:MaxTenuringThreshold=1 -XX:+UseStringDeduplication -XX:+OptimizeStringConcat -XX:+UseFastUnorderedTimeStamps -XX:+UseVectorCmov -Dfabric.log.disable=false -Djava.awt.headless=true -Dorg.lwjgl.opengl.Display.enableOSX=false",
            _ => "-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200 -XX:+UnlockExperimentalVMOptions -XX:+DisableExplicitGC -XX:+AlwaysPreTouch -XX:G1NewSizePercent=30 -XX:G1MaxNewSizePercent=40 -XX:G1ReservePercent=20 -XX:G1HeapWastePercent=5 -XX:G1MixedGCCountTarget=4 -XX:InitiatingHeapOccupancyPercent=15 -XX:G1PeriodicGCInterval=60000",
        }
    }
}
