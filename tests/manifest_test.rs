use nux_launcher::core::manifest::Library;

#[test]
fn test_library_rule_evaluation() {
    let lib_vanilla = Library {
        name: "com.mojang:patchy:1.3.9".to_string(),
        downloads: None,
        rules: None,
        natives: None,
    };
    assert!(lib_vanilla.is_applicable_for_linux());

    let path = lib_vanilla.maven_to_path().unwrap();
    assert_eq!(path, "com/mojang/patchy/1.3.9/patchy-1.3.9.jar");
}

#[test]
fn test_library_artifact_path_generation() {
    let lib = Library {
        name: "net.fabricmc:fabric-loader:0.15.11".to_string(),
        downloads: None,
        rules: None,
        natives: None,
    };
    let path = lib.artifact_path().unwrap();
    assert_eq!(path, "net/fabricmc/fabric-loader/0.15.11/fabric-loader-0.15.11.jar");
}
