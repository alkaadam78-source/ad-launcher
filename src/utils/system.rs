use sysinfo::System;

#[derive(Debug, Clone)]
pub struct SystemSpecs {
    pub total_memory_mb: u64,
    pub available_memory_mb: u64,
    pub cpu_name: String,
    pub cpu_cores: usize,
    pub os_name: String,
    pub os_version: String,
    pub recommended_ram_mb: u32,
    pub max_safe_ram_mb: u32,
}

pub fn detect_system_specs() -> SystemSpecs {
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_ram_mb = sys.total_memory() / 1024 / 1024;
    let avail_ram_mb = sys.available_memory() / 1024 / 1024;

    let cpu_name = sys
        .cpus()
        .first()
        .map(|c| c.brand().trim().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string());

    let cpu_cores = sys.cpus().len();

    let os_name = System::name().unwrap_or_else(|| "Linux".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Debian".to_string());

    // Calculate safe RAM for Minecraft
    // On 8GB machine (e.g. i3-7100U + HD 620), 3072MB is optimal.
    // On 4GB machine, 2048MB is maximum safe.
    let recommended_ram_mb = if total_ram_mb <= 4096 {
        2048
    } else if total_ram_mb <= 8192 {
        3072
    } else {
        4096
    };

    // Hard cap at 70% of total memory to prevent OOM killer on Linux
    let max_safe_ram_mb = ((total_ram_mb as f64) * 0.70) as u32;

    SystemSpecs {
        total_memory_mb: total_ram_mb,
        available_memory_mb: avail_ram_mb,
        cpu_name,
        cpu_cores,
        os_name,
        os_version,
        recommended_ram_mb,
        max_safe_ram_mb,
    }
}
