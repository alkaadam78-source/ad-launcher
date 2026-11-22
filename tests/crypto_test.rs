use ad_launcher::utils::crypto::{bytes_sha1, verify_sha1};
use std::io::Write;

#[test]
fn test_bytes_sha1() {
    let input = b"minecraft-1.20.1";
    let hash = bytes_sha1(input);
    assert_eq!(hash.len(), 40);
}

#[test]
fn test_file_sha1_verification() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("nux_test_file.txt");

    let content = b"nux-launcher-verification-test";
    let expected_hash = bytes_sha1(content);

    let mut f = std::fs::File::create(&test_file).unwrap();
    f.write_all(content).unwrap();
    drop(f);

    assert!(verify_sha1(&test_file, &expected_hash));
    assert!(!verify_sha1(
        &test_file,
        "0000000000000000000000000000000000000000"
    ));

    let _ = std::fs::remove_file(test_file);
}
