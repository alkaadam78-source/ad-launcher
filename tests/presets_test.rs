use nux_launcher::config::presets::OptimizationPreset;

#[test]
fn test_balanced_preset_parameters() {
    let preset = OptimizationPreset::Balanced;
    assert_eq!(preset.default_ram_mb(), 3072);
    assert_eq!(preset.render_distance(), 6);
    assert_eq!(preset.simulation_distance(), 4);
    assert_eq!(preset.max_fps(), 120);
    assert!(preset.recommended_jvm_flags().contains("UseG1GC"));
}

#[test]
fn test_low_resource_preset_parameters() {
    let preset = OptimizationPreset::LowResource;
    assert_eq!(preset.default_ram_mb(), 2048);
    assert_eq!(preset.render_distance(), 4);
    assert_eq!(preset.simulation_distance(), 3);
    assert_eq!(preset.max_fps(), 90);
}
