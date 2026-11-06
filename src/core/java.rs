use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JavaRuntime {
    pub path: PathBuf,
    pub version_string: String,
    pub major_version: u32,
    pub is_64bit: bool,
    pub is_compatible_1_20_1: bool,
}

impl JavaRuntime {
    pub fn probe<P: AsRef<Path>>(binary_path: P) -> Option<Self> {
        let path = binary_path.as_ref();
        if !path.exists() {
            return None;
        }

        let output = Command::new(path).arg("-version").output().ok()?;

        // java -version prints to stderr
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let full_output = format!("{}\n{}", stderr, stdout);

        let (version_string, major) = parse_java_version(&full_output)?;
        let is_64bit = full_output.to_lowercase().contains("64-bit")
            || full_output.contains("x86_64")
            || full_output.contains("amd64");

        let is_compatible_1_20_1 = major >= 17;

        Some(Self {
            path: path.to_path_buf(),
            version_string,
            major_version: major,
            is_64bit,
            is_compatible_1_20_1,
        })
    }
}

fn parse_java_version(output: &str) -> Option<(String, u32)> {
    for line in output.lines() {
        if line.contains("version") {
            // Examples:
            // openjdk version "17.0.21" 2026-10-20
            // java version "1.8.0_312"
            if let Some(start) = line.find('"') {
                if let Some(end) = line[start + 1..].find('"') {
                    let ver = &line[start + 1..start + 1 + end];
                    let major = if ver.starts_with("1.") {
                        ver.split('.').nth(1)?.parse::<u32>().ok()?
                    } else {
                        ver.split('.').next()?.parse::<u32>().ok()?
                    };
                    return Some((ver.to_string(), major));
                }
            }
        }
    }
    None
}

/// Discovers installed Java runtimes on the Debian Linux host
pub fn discover_java_runtimes(custom_path: Option<&Path>) -> Vec<JavaRuntime> {
    let mut candidates = Vec::new();

    if let Some(custom) = custom_path {
        candidates.push(custom.to_path_buf());
    }

    // Minecraft bundled runtimes in ~/.minecraft/runtime/
    if let Some(home) = dirs::home_dir() {
        let mc_runtime = home.join(".minecraft").join("runtime");
        if mc_runtime.exists() {
            candidates.push(mc_runtime.join("java-17/bin/java"));
            candidates.push(mc_runtime.join("java-runtime-gamma/bin/java"));
            candidates.push(mc_runtime.join("java-runtime-beta/bin/java"));
            candidates.push(mc_runtime.join("java-runtime-alpha/bin/java"));

            // Check any other subdirectories
            if let Ok(entries) = std::fs::read_dir(&mc_runtime) {
                for entry in entries.flatten() {
                    let java_bin = entry.path().join("bin/java");
                    if java_bin.exists() {
                        candidates.push(java_bin);
                    }
                }
            }
        }
    }

    // System JVM directories on Debian
    let jvm_dir = Path::new("/usr/lib/jvm");
    if jvm_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(jvm_dir) {
            for entry in entries.flatten() {
                let java_bin = entry.path().join("bin/java");
                if java_bin.exists() {
                    candidates.push(java_bin);
                }
            }
        }
    }

    // System PATH java
    if let Ok(output) = Command::new("which").arg("java").output() {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                candidates.push(PathBuf::from(path_str));
            }
        }
    }

    let mut found = Vec::new();
    let mut seen_paths = std::collections::HashSet::new();

    for candidate in candidates {
        if let Ok(canonical) = candidate.canonicalize() {
            if seen_paths.insert(canonical.clone()) {
                if let Some(runtime) = JavaRuntime::probe(&canonical) {
                    found.push(runtime);
                }
            }
        }
    }

    // Sort to prioritize Java 17 64-bit first, then other Java >= 17, then others
    found.sort_by(|a, b| {
        let score_a = if a.major_version == 17 && a.is_64bit {
            100
        } else if a.is_compatible_1_20_1 && a.is_64bit {
            80
        } else if a.is_compatible_1_20_1 {
            50
        } else {
            10
        };

        let score_b = if b.major_version == 17 && b.is_64bit {
            100
        } else if b.is_compatible_1_20_1 && b.is_64bit {
            80
        } else if b.is_compatible_1_20_1 {
            50
        } else {
            10
        };

        score_b.cmp(&score_a)
    });

    found
}
