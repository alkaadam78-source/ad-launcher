use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct JavaRuntimeInfo {
    pub path: PathBuf,
    pub version_string: String,
    pub major_version: u32,
    pub vendor: String,
    pub is_64bit: bool,
    pub is_recommended: bool,
}

pub struct JavaManager;

impl JavaManager {
    /// Comprehensive discovery of Java installations across Debian and standard Linux distros
    pub fn discover_all_runtimes() -> Vec<JavaRuntimeInfo> {
        let mut runtimes = Vec::new();
        let mut candidates = Vec::new();

        // 1. Standard JVM search paths on Debian/Ubuntu/Fedora/Arch
        let search_dirs = [
            "/usr/lib/jvm",
            "/usr/lib64/jvm",
            "/opt/java",
            "/opt/jdk",
            "/usr/java",
        ];

        for dir in search_dirs {
            let p = Path::new(dir);
            if p.exists() && p.is_dir() {
                if let Ok(entries) = std::fs::read_dir(p) {
                    for entry in entries.flatten() {
                        let java_bin = entry.path().join("bin/java");
                        if java_bin.exists() && java_bin.is_file() {
                            candidates.push(java_bin);
                        }
                    }
                }
            }
        }

        // 2. User local JDKs (~/.jdks, ~/.sdkman/candidates/java/*)
        if let Some(home) = dirs::home_dir() {
            let user_dirs = [
                home.join(".jdks"),
                home.join(".sdkman/candidates/java"),
                home.join(".local/share/java"),
            ];
            for udir in user_dirs {
                if udir.exists() && udir.is_dir() {
                    if let Ok(entries) = std::fs::read_dir(&udir) {
                        for entry in entries.flatten() {
                            let java_bin = entry.path().join("bin/java");
                            if java_bin.exists() && java_bin.is_file() {
                                candidates.push(java_bin);
                            }
                        }
                    }
                }
            }
        }

        // 3. System `which java`
        if let Ok(output) = Command::new("which").arg("java").output() {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path_str.is_empty() {
                    let pb = PathBuf::from(&path_str);
                    if let Ok(canonical) = std::fs::canonicalize(&pb) {
                        candidates.push(canonical);
                    } else {
                        candidates.push(pb);
                    }
                }
            }
        }

        // Deduplicate candidates
        candidates.sort();
        candidates.dedup();

        for candidate in candidates {
            if let Some(info) = Self::probe_java_binary(&candidate) {
                runtimes.push(info);
            }
        }

        // Sort: Recommended first, then by major version descending
        runtimes.sort_by(|a, b| {
            b.is_recommended
                .cmp(&a.is_recommended)
                .then(b.major_version.cmp(&a.major_version))
        });

        runtimes
    }

    pub fn probe_java_binary(path: &Path) -> Option<JavaRuntimeInfo> {
        let output = Command::new(path).arg("-version").output().ok()?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let combined = format!("{}\n{}", stderr, stdout);

        let (major, ver_str) = Self::parse_java_version(&combined);
        let vendor = Self::parse_vendor(&combined);
        let is_64bit = combined.contains("64-Bit")
            || combined.contains("x86_64")
            || combined.contains("amd64");
        let is_recommended = major == 17 || major == 21;

        Some(JavaRuntimeInfo {
            path: path.to_path_buf(),
            version_string: ver_str,
            major_version: major,
            vendor,
            is_64bit,
            is_recommended,
        })
    }

    fn parse_java_version(output: &str) -> (u32, String) {
        for line in output.lines() {
            if line.contains("version") {
                if let Some(start) = line.find('"') {
                    if let Some(end) = line[start + 1..].find('"') {
                        let ver_str = &line[start + 1..start + 1 + end];
                        let parts: Vec<&str> = ver_str.split('.').collect();
                        if !parts.is_empty() {
                            if parts[0] == "1" && parts.len() > 1 {
                                if let Ok(v) = parts[1].parse::<u32>() {
                                    return (v, ver_str.to_string());
                                }
                            } else if let Ok(v) = parts[0].parse::<u32>() {
                                return (v, ver_str.to_string());
                            }
                        }
                        return (17, ver_str.to_string());
                    }
                }
            }
        }
        (17, "17.0.x (Detected)".to_string())
    }

    fn parse_vendor(output: &str) -> String {
        if output.contains("Temurin") {
            "Eclipse Temurin".to_string()
        } else if output.contains("OpenJDK") {
            "OpenJDK Community".to_string()
        } else if output.contains("GraalVM") {
            "GraalVM Native".to_string()
        } else if output.contains("Oracle") {
            "Oracle JDK".to_string()
        } else {
            "Standard JVM".to_string()
        }
    }
}
