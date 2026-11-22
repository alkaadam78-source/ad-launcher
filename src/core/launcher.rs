use crate::config::settings::LauncherSettings;
use crate::core::authentication::{AccountProfile, AuthManager};
use crate::core::instances::{Instance, InstanceManager};
use crate::core::java::{discover_java_runtimes, JavaRuntime};
use crate::core::minecraft::MinecraftEngine;
use crate::core::mods::{ModManager, PerformanceModStatus};
use crate::core::process::ProcessEvent;
use std::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub struct LauncherStatus {
    pub is_ready: bool,
    pub status_message: String,
    pub selected_instance: Option<Instance>,
    pub active_account: AccountProfile,
    pub detected_java: Option<JavaRuntime>,
    pub performance_status: PerformanceModStatus,
}

pub struct LauncherCore {
    pub settings: LauncherSettings,
    pub instance_manager: InstanceManager,
    pub auth_manager: AuthManager,
}

impl LauncherCore {
    pub fn init() -> Result<Self, String> {
        let settings = LauncherSettings::load();
        let instance_manager = InstanceManager::new().map_err(|e| e.to_string())?;
        let auth_manager = AuthManager::new();

        Ok(Self {
            settings,
            instance_manager,
            auth_manager,
        })
    }

    pub fn get_active_account(&self) -> AccountProfile {
        self.auth_manager.load_active_account().unwrap_or_else(|| {
            self.auth_manager
                .create_offline_profile(&self.settings.active_profile_name)
        })
    }

    pub fn get_selected_instance(&self) -> Option<Instance> {
        let instances = self.instance_manager.list_instances().unwrap_or_default();
        instances
            .into_iter()
            .find(|i| i.id == self.settings.selected_instance)
            .or_else(|| {
                self.instance_manager
                    .list_instances()
                    .ok()
                    .and_then(|list| list.into_iter().next())
            })
    }

    pub fn get_status(&self) -> LauncherStatus {
        let selected_instance = self.get_selected_instance();
        let active_account = self.get_active_account();
        let runtimes = discover_java_runtimes(self.settings.java_path.as_deref());
        let detected_java = runtimes.into_iter().find(|r| r.is_compatible_1_20_1);

        let mc_dir = &self.settings.minecraft_dir;
        let mod_manager = ModManager::new(
            selected_instance
                .as_ref()
                .map(|i| i.game_dir.as_path())
                .unwrap_or(mc_dir.as_path()),
        );
        let performance_status = mod_manager.check_core_performance_mods();

        let (is_ready, status_message) = if selected_instance.is_none() {
            (false, "No instance configured".to_string())
        } else if detected_java.is_none() {
            (
                false,
                "No compatible Java 17+ 64-bit runtime found".to_string(),
            )
        } else {
            let inst = selected_instance.as_ref().unwrap();
            let v_jar = mc_dir
                .join("versions")
                .join(&inst.mc_version)
                .join(format!("{}.jar", inst.mc_version));

            if !v_jar.exists() {
                (
                    false,
                    format!("Minecraft {} is not downloaded", inst.mc_version),
                )
            } else {
                (true, format!("Ready to Launch ({})", inst.name))
            }
        };

        LauncherStatus {
            is_ready,
            status_message,
            selected_instance,
            active_account,
            detected_java,
            performance_status,
        }
    }

    pub async fn launch_current(&mut self, event_tx: Sender<ProcessEvent>) -> Result<u32, String> {
        let instance = self
            .get_selected_instance()
            .ok_or_else(|| "No instance selected".to_string())?;

        let account = self.get_active_account();
        let engine = MinecraftEngine::new(&self.settings.minecraft_dir);

        let pid = engine
            .launch(&instance, &self.settings, &account, event_tx)
            .await?;

        // Update last played timestamp in database
        let _ = self.instance_manager.update_last_played(&instance.id);

        Ok(pid)
    }
}
