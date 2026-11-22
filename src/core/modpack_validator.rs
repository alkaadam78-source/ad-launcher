use crate::core::mods::ModInfo;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ModValidationIssue {
    pub level: IssueSeverity,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub total_mods_scanned: usize,
    pub is_valid: bool,
    pub issues: Vec<ModValidationIssue>,
    pub recommendations: Vec<String>,
}

pub struct ModpackValidator;

impl ModpackValidator {
    /// Validates an installed mod list against version, dependencies, duplicates, and conflicts
    pub fn validate_mods(mods: &[ModInfo], mc_version: &str, loader: &str) -> ValidationReport {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        let total = mods.len();
        let mut name_map: HashMap<String, Vec<&ModInfo>> = HashMap::new();
        let mut known_mods = HashSet::new();

        for m in mods {
            let key = m.name.to_lowercase().replace(['-', '_', ' '], "");
            name_map.entry(key.clone()).or_default().push(m);
            known_mods.insert(m.name.to_lowercase());
        }

        // 1. Check for duplicate versions of same mod
        for (name, entries) in name_map {
            if entries.len() > 1 {
                issues.push(ModValidationIssue {
                    level: IssueSeverity::Critical,
                    title: format!("Duplicate Mod Detected: {}", name),
                    description: format!(
                        "Multiple jars exist for '{}': {}. Remove older versions to prevent crash.",
                        name,
                        entries
                            .iter()
                            .map(|e| e.file_name.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                });
            }
        }

        // 2. Check for Essential Fabric Dependencies
        if loader.to_lowercase() == "fabric" && total > 0 {
            let has_fabric_api = known_mods
                .iter()
                .any(|m| m.contains("fabricapi") || m.contains("fabric-api"));
            if !has_fabric_api {
                issues.push(ModValidationIssue {
                    level: IssueSeverity::Warning,
                    title: "Missing Fabric API".to_string(),
                    description: "Fabric mods are installed, but Fabric API was not detected. Many mods require Fabric API to initialize.".to_string(),
                });
                recommendations.push("Install Fabric API from the Mods Archive.".to_string());
            }
        }

        // 3. Check for Potato Optimization Synergy (Intel HD 620 target)
        let has_sodium = known_mods
            .iter()
            .any(|m| m.contains("sodium") || m.contains("embeddium"));
        let has_iris = known_mods
            .iter()
            .any(|m| m.contains("iris") || m.contains("oculus"));
        let has_lithium = known_mods.iter().any(|m| m.contains("lithium"));
        let has_ferrite = known_mods.iter().any(|m| m.contains("ferritecore"));

        if !has_sodium && mc_version.starts_with("1.2") {
            recommendations.push(
                "Commission Sodium / Embeddium for 3x - 5x FPS on Intel HD Graphics.".to_string(),
            );
        }
        if has_iris && !has_sodium {
            issues.push(ModValidationIssue {
                level: IssueSeverity::Warning,
                title: "Iris Shaders Without Sodium".to_string(),
                description: "Iris shaders require Sodium for rendering pipeline hooks."
                    .to_string(),
            });
        }
        if !has_lithium && total > 5 {
            recommendations
                .push("Add Lithium for server physics & tick loop optimization.".to_string());
        }
        if !has_ferrite && total > 15 {
            recommendations
                .push("Add FerriteCore to reduce memory allocation footprint by ~40%.".to_string());
        }

        let is_valid = !issues.iter().any(|i| i.level == IssueSeverity::Critical);

        ValidationReport {
            total_mods_scanned: total,
            is_valid,
            issues,
            recommendations,
        }
    }
}
