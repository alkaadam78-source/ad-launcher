use nux_launcher::core::java::discover_java_runtimes;

#[test]
fn test_java_detection_finds_runtime() {
    let runtimes = discover_java_runtimes(None);
    assert!(!runtimes.is_empty(), "Expected at least one Java runtime to be discovered");

    for rt in &runtimes {
        println!("Found Java: {} (major {}) at {:?}", rt.version_string, rt.major_version, rt.path);
        assert!(rt.path.exists());
        assert!(rt.major_version > 0);
    }
}

#[test]
fn test_java_compatibility_for_1_20_1() {
    let runtimes = discover_java_runtimes(None);
    let compatible = runtimes.iter().any(|r| r.is_compatible_1_20_1);
    assert!(compatible, "Expected to find a Java runtime compatible with 1.20.1 (>= 17)");
}
