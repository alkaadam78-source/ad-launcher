use ad_launcher::core::mods::ModManager;

#[test]
fn test_known_performance_mods_detection() {
    assert!(ModManager::is_known_performance_mod("sodium", "Sodium"));
    assert!(ModManager::is_known_performance_mod("iris", "Iris"));
    assert!(ModManager::is_known_performance_mod("lithium", "Lithium"));
    assert!(ModManager::is_known_performance_mod(
        "ferritecore",
        "FerriteCore"
    ));
    assert!(ModManager::is_known_performance_mod("indium", "Indium"));
    assert!(ModManager::is_known_performance_mod(
        "modernfix",
        "ModernFix"
    ));
    assert!(!ModManager::is_known_performance_mod(
        "appleskin",
        "AppleSkin"
    ));
}
