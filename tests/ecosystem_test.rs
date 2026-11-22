use ad_launcher::config::settings::LauncherSettings;
use ad_launcher::core::diagnostics::generate_sanitized_report;
use ad_launcher::core::i18n::{t, Language};
use ad_launcher::core::importer::InstanceImporter;
use ad_launcher::core::modpack_validator::ModpackValidator;
use ad_launcher::core::mods::ModInfo;
use ad_launcher::core::updater::{AppUpdater, UpdateChannel};
use ad_launcher::utils::system::detect_system_specs;

#[test]
fn test_i18n_translation_lookup() {
    assert_eq!(t("app.subtitle", Language::English), "THE ART OF MINECRAFT");
    assert_eq!(t("app.subtitle", Language::Indonesian), "SENI MINECRAFT");
    assert_eq!(t("unknown_key_xyz", Language::English), "unknown_key_xyz");
}

#[test]
fn test_diagnostics_sanitization() {
    let specs = detect_system_specs();
    let settings = LauncherSettings::default();
    let report = generate_sanitized_report(&specs, &settings);

    assert!(report.contains("AD LAUNCHER — SANITIZED DIAGNOSTIC"));
    assert!(report.contains("0.1.0-beta.1"));
    // Ensure no auth tokens
    assert!(!report.contains("access_token"));
    assert!(!report.contains("refresh_token"));
}

#[test]
fn test_modpack_validator_duplicate_detection() {
    let mods = vec![
        ModInfo {
            path: std::path::PathBuf::from("/mods/sodium-0.5.8.jar"),
            id: "sodium".to_string(),
            name: "Sodium".to_string(),
            version: "0.5.8".to_string(),
            description: "Rendering engine".to_string(),
            authors: vec!["JellySquid".to_string()],
            file_name: "sodium-0.5.8.jar".to_string(),
            enabled: true,
            is_performance_mod: true,
        },
        ModInfo {
            path: std::path::PathBuf::from("/mods/sodium-0.5.11.jar"),
            id: "sodium".to_string(),
            name: "Sodium".to_string(),
            version: "0.5.11".to_string(),
            description: "Rendering engine".to_string(),
            authors: vec!["JellySquid".to_string()],
            file_name: "sodium-0.5.11.jar".to_string(),
            enabled: true,
            is_performance_mod: true,
        },
    ];

    let report = ModpackValidator::validate_mods(&mods, "1.20.1", "fabric");
    assert!(!report.is_valid);
    assert!(report
        .issues
        .iter()
        .any(|i| i.title.contains("Duplicate Mod Detected")));
}

#[test]
fn test_app_updater_channel() {
    let update = AppUpdater::check_for_updates(UpdateChannel::Beta);
    assert_eq!(update.version, "0.1.0-beta.1");
    assert!(!update.has_update);
}

#[test]
fn test_instance_importer_scan_safe() {
    // Should scan without crashing or raising errors
    let discovered = InstanceImporter::scan_for_importable_instances();
    let _ = discovered.len();
}
