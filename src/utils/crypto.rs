use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Calculates SHA-1 checksum of a file as a lowercase hex string
pub fn file_sha1<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha1::new();
    let mut buffer = [0u8; 64 * 1024];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Verifies whether the file exists and matches the expected SHA-1
pub fn verify_sha1<P: AsRef<Path>>(path: P, expected_sha1: &str) -> bool {
    let path = path.as_ref();
    if !path.exists() {
        return false;
    }
    match file_sha1(path) {
        Ok(calculated) => calculated.eq_ignore_ascii_case(expected_sha1),
        Err(_) => false,
    }
}

/// Calculates SHA-1 of byte slice
pub fn bytes_sha1(bytes: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    format!("{:x}", result)
}
