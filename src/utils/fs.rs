use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

/// Returns standard .minecraft directory on Linux
pub fn default_minecraft_dir() -> PathBuf {
    dirs::home_dir()
        .map(|h| h.join(".minecraft"))
        .unwrap_or_else(|| PathBuf::from("./.minecraft"))
}

/// Returns launcher config directory (~/.config/nux-launcher)
pub fn launcher_config_dir() -> PathBuf {
    dirs::config_dir()
        .map(|c| c.join("nux-launcher"))
        .unwrap_or_else(|| PathBuf::from("./.nux-launcher"))
}

/// Ensures directory exists
pub fn ensure_dir<P: AsRef<Path>>(dir: P) -> io::Result<()> {
    fs::create_dir_all(dir.as_ref())
}

/// Extracts a zip archive to destination, preserving subdirectories
pub fn extract_zip<P: AsRef<Path>, D: AsRef<Path>>(archive_path: P, dest_dir: D) -> io::Result<()> {
    let file = File::open(archive_path)?;
    let reader = BufReader::new(file);
    let mut archive =
        ZipArchive::new(reader).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let dest = dest_dir.as_ref();
    ensure_dir(dest)?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let outpath = match file.enclosed_name() {
            Some(path) => dest.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            ensure_dir(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                ensure_dir(p)?;
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}
