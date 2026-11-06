use crate::config::settings::LauncherSettings;
use crate::core::authentication::AccountProfile;
use crate::core::instances::Instance;
use crate::core::java::{discover_java_runtimes, JavaRuntime};
use crate::core::manifest::VersionPackage;
use crate::core::process::{LaunchConfig, MinecraftProcess, ProcessEvent};
use crate::utils::fs::{ensure_dir, extract_zip};
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;

pub struct MinecraftEngine {
    minecraft_dir: PathBuf,
}

impl MinecraftEngine {
    pub fn new<P: AsRef<Path>>(minecraft_dir: P) -> Self {
        Self {
            minecraft_dir: minecraft_dir.as_ref().to_path_buf(),
        }
    }

    /// Prepares and launches Minecraft 1.20.1 (Vanilla or Fabric)
    pub async fn launch(
        &self,
        instance: &Instance,
        settings: &LauncherSettings,
        account: &AccountProfile,
        event_tx: Sender<ProcessEvent>,
    ) -> Result<u32, String> {
        // 1. Pick Java Runtime
        let java_runtime = self.resolve_java_runtime(instance, settings)?;
        let _ = event_tx.send(ProcessEvent::LogLine(format!(
            "[NUX] Using Java {} ({}) at {}",
            java_runtime.version_string,
            if java_runtime.is_64bit { "64-bit" } else { "32-bit" },
            java_runtime.path.display()
        )));

        // 2. Resolve version details
        let version_id = &instance.mc_version;
        let version_dir = self.minecraft_dir.join("versions").join(version_id);
        let version_json_path = version_dir.join(format!("{}.json", version_id));
        let client_jar_path = version_dir.join(format!("{}.jar", version_id));

        if !version_json_path.exists() || !client_jar_path.exists() {
            return Err(format!(
                "Minecraft {} is not fully downloaded. Please install it first from Downloads.",
                version_id
            ));
        }

        let version_json_content = std::fs::read_to_string(&version_json_path)
            .map_err(|e| format!("Failed to read version JSON: {}", e))?;
        let pkg: VersionPackage = serde_json::from_str(&version_json_content)
            .map_err(|e| format!("Failed to parse version JSON: {}", e))?;

        // 3. Resolve Classpath & Natives
        let mut classpath = Vec::new();
        let libs_dir = self.minecraft_dir.join("libraries");
        let natives_dir = self.minecraft_dir.join("natives").join(version_id);
        ensure_dir(&natives_dir).map_err(|e| e.to_string())?;

        for lib in &pkg.libraries {
            if !lib.is_applicable_for_linux() {
                continue;
            }

            if let Some(rel) = lib.artifact_path() {
                let lib_path = libs_dir.join(&rel);
                if lib_path.exists() {
                    classpath.push(lib_path);
                }
            }

            // Check if there are Linux native classifiers to extract
            if let Some(downloads) = &lib.downloads {
                if let Some(classifiers) = &downloads.classifiers {
                    if let Some(native_art) = classifiers.get("natives-linux") {
                        if let Some(rel) = &native_art.path {
                            let native_jar = libs_dir.join(rel);
                            if native_jar.exists() {
                                let _ = extract_zip(&native_jar, &natives_dir);
                            }
                        }
                    }
                }
            }
        }

        // Add client jar
        classpath.push(client_jar_path);

        // 4. Fabric Loader integration (if instance loader is fabric)
        let main_class = if instance.loader == "fabric" {
            // Check if Fabric loader profile exists or if fabric libraries exist in libraries/net/fabricmc
            let fabric_profile_path = self
                .minecraft_dir
                .join("versions")
                .join(format!("fabric-loader-{}", instance.mc_version))
                .join(format!("fabric-loader-{}.json", instance.mc_version));

            // Also check alternate Fabric profiles in versions/
            let alt_fabric = self.find_existing_fabric_version(version_id);

            if let Some(fabric_path) = alt_fabric.or_else(|| {
                if fabric_profile_path.exists() {
                    Some(fabric_profile_path)
                } else {
                    None
                }
            }) {
                self.merge_fabric_profile(&fabric_path, &mut classpath)?;
            }

            // Fabric knot main class
            "net.fabricmc.loader.impl.launch.knot.KnotClient".to_string()
        } else {
            pkg.main_class.clone()
        };

        // 5. Build JVM arguments
        let mut jvm_args = Vec::new();
        let ram_mb = instance.ram_mb.max(1024);
        jvm_args.push(format!("-Xms512M"));
        jvm_args.push(format!("-Xmx{}M", ram_mb));

        // GC & Memory optimization for i3-7100U + 8GB Debian Linux
        if settings.enable_performance_gc {
            for flag in settings.preset.recommended_jvm_flags().split_whitespace() {
                jvm_args.push(flag.to_string());
            }
        }

        // Custom JVM args
        if !settings.custom_jvm_args.trim().is_empty() {
            for arg in settings.custom_jvm_args.split_whitespace() {
                jvm_args.push(arg.to_string());
            }
        }

        // Native library path
        jvm_args.push(format!("-Djava.library.path={}", natives_dir.display()));

        // Linux display integration & Wayland/X11 compatibility flags
        jvm_args.push("-Dorg.lwjgl.opengl.Display.enableOSX=false".to_string());

        // 6. Build Game arguments
        let mut game_args = Vec::new();
        game_args.push("--username".to_string());
        game_args.push(account.username.clone());
        game_args.push("--version".to_string());
        game_args.push(version_id.clone());
        game_args.push("--gameDir".to_string());
        game_args.push(instance.game_dir.to_string_lossy().to_string());
        game_args.push("--assetsDir".to_string());
        game_args.push(self.minecraft_dir.join("assets").to_string_lossy().to_string());
        game_args.push("--assetIndex".to_string());
        game_args.push(pkg.asset_index.id.clone());
        game_args.push("--uuid".to_string());
        game_args.push(account.uuid.clone());
        game_args.push("--accessToken".to_string());
        game_args.push(account.access_token.clone());
        game_args.push("--userType".to_string());
        game_args.push(if account.is_microsoft { "msa" } else { "mojang" }.to_string());
        game_args.push("--versionType".to_string());
        game_args.push("release".to_string());

        // Resolution
        if settings.window_width > 0 && settings.window_height > 0 {
            game_args.push("--width".to_string());
            game_args.push(settings.window_width.to_string());
            game_args.push("--height".to_string());
            game_args.push(settings.window_height.to_string());
        }

        let config = LaunchConfig {
            java_binary: java_runtime.path,
            jvm_args,
            classpath,
            main_class,
            game_args,
            game_dir: instance.game_dir.clone(),
        };

        let pid = MinecraftProcess::spawn(config, event_tx)?;
        Ok(pid)
    }

    fn resolve_java_runtime(
        &self,
        instance: &Instance,
        settings: &LauncherSettings,
    ) -> Result<JavaRuntime, String> {
        if let Some(ref path) = instance.java_path {
            if let Some(rt) = JavaRuntime::probe(path) {
                return Ok(rt);
            }
        }
        if let Some(ref path) = settings.java_path {
            if let Some(rt) = JavaRuntime::probe(path) {
                return Ok(rt);
            }
        }

        let runtimes = discover_java_runtimes(None);
        runtimes
            .into_iter()
            .find(|r| r.is_compatible_1_20_1)
            .ok_or_else(|| {
                "No compatible Java 17+ 64-bit runtime found. Please install OpenJDK 17.".to_string()
            })
    }

    fn find_existing_fabric_version(&self, _mc_version: &str) -> Option<PathBuf> {
        let versions_dir = self.minecraft_dir.join("versions");
        if let Ok(entries) = std::fs::read_dir(versions_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.contains("Fabric") || name.contains("fabric-loader") {
                    let json_path = entry.path().join(format!("{}.json", name));
                    if json_path.exists() {
                        return Some(json_path);
                    }
                }
            }
        }
        None
    }

    fn merge_fabric_profile(
        &self,
        fabric_json_path: &Path,
        classpath: &mut Vec<PathBuf>,
    ) -> Result<(), String> {
        let content = std::fs::read_to_string(fabric_json_path)
            .map_err(|e| format!("Failed to read Fabric profile: {}", e))?;
        let json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse Fabric profile: {}", e))?;

        let libs_dir = self.minecraft_dir.join("libraries");

        if let Some(libs) = json["libraries"].as_array() {
            for lib in libs {
                if let Some(name) = lib["name"].as_str() {
                    // Convert maven coordinate to path
                    let parts: Vec<&str> = name.split(':').collect();
                    if parts.len() >= 3 {
                        let group = parts[0].replace('.', "/");
                        let artifact = parts[1];
                        let version = parts[2];
                        let jar_name = format!("{}-{}.jar", artifact, version);
                        let lib_path = libs_dir.join(group).join(artifact).join(version).join(jar_name);
                        if lib_path.exists() && !classpath.contains(&lib_path) {
                            classpath.insert(0, lib_path); // Fabric loader jars should be first in classpath
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
