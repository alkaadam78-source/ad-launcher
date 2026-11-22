use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginPermission {
    NetworkAccess,
    FilesystemRead,
    FilesystemWrite,
    ShadersAccess,
    DiagnosticsRead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub requested_permissions: Vec<PluginPermission>,
    pub is_enabled: bool,
}

pub struct PluginManager;

impl PluginManager {
    /// Discovers bundled or user-provided extensions
    pub fn list_curated_extensions() -> Vec<PluginManifest> {
        vec![
            PluginManifest {
                id: "ext.baroque-shaders".to_string(),
                name: "Baroque Chiaroscuro Shader Suite".to_string(),
                version: "1.0.0".to_string(),
                author: "AdamDevnoob".to_string(),
                description: "Lightweight shader preset hooks tuned for OpenGL 4.6 on integrated Intel HD Graphics 620.".to_string(),
                requested_permissions: vec![PluginPermission::ShadersAccess, PluginPermission::FilesystemRead],
                is_enabled: true,
            },
            PluginManifest {
                id: "ext.potato-telemetry".to_string(),
                name: "Low-Spec Potato Hardware Telemetry".to_string(),
                version: "1.0.0".to_string(),
                author: "AD Engineering".to_string(),
                description: "Mesa glthread and thread priority supervisor for smooth 60+ FPS gameplay.".to_string(),
                requested_permissions: vec![PluginPermission::DiagnosticsRead],
                is_enabled: true,
            },
        ]
    }
}
