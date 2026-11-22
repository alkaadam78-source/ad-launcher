use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceExportManifest {
    pub format_version: u32,
    pub instance_name: String,
    pub mc_version: String,
    pub loader: String,
    pub loader_version: String,
    pub ram_mb: u32,
    pub created_at: String,
    pub exported_by: String,
}

pub struct InstancePorter;

impl InstancePorter {
    /// Creates a portable zip archive of an instance
    pub fn export_instance_to_archive(
        instance_dir: &Path,
        manifest: &InstanceExportManifest,
        output_zip_path: &Path,
        include_worlds: bool,
    ) -> std::io::Result<u64> {
        let file = File::create(output_zip_path)?;
        let mut zip = ZipWriter::new(file);
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        // 1. Write ad-instance.json manifest
        zip.start_file("ad-instance.json", options)?;
        let manifest_bytes = serde_json::to_vec_pretty(manifest)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        zip.write_all(&manifest_bytes)?;

        // 2. Add mods
        let mods_dir = instance_dir.join("mods");
        if mods_dir.exists() && mods_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(&mods_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |e| e == "jar") {
                        let filename = entry.file_name();
                        let zip_name = format!("mods/{}", filename.to_string_lossy());
                        zip.start_file(zip_name, options)?;
                        let mut f = File::open(&path)?;
                        let mut buf = Vec::new();
                        f.read_to_end(&mut buf)?;
                        zip.write_all(&buf)?;
                    }
                }
            }
        }

        // 3. Add options.txt if exists
        let options_file = instance_dir.join("options.txt");
        if options_file.exists() && options_file.is_file() {
            zip.start_file("options.txt", options)?;
            let mut f = File::open(&options_file)?;
            let mut buf = Vec::new();
            f.read_to_end(&mut buf)?;
            zip.write_all(&buf)?;
        }

        // 4. Optionally add saves
        if include_worlds {
            let saves_dir = instance_dir.join("saves");
            if saves_dir.exists() {
                // Add save files
                Self::add_dir_to_zip(&mut zip, &saves_dir, "saves", options)?;
            }
        }

        zip.finish()?;
        let meta = fs::metadata(output_zip_path)?;
        Ok(meta.len())
    }

    fn add_dir_to_zip<W: Write + std::io::Seek>(
        zip: &mut ZipWriter<W>,
        dir: &Path,
        prefix: &str,
        options: SimpleFileOptions,
    ) -> std::io::Result<()> {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name();
                let zip_path = format!("{}/{}", prefix, name.to_string_lossy());
                if path.is_dir() {
                    Self::add_dir_to_zip(zip, &path, &zip_path, options)?;
                } else if path.is_file() {
                    zip.start_file(zip_path, options)?;
                    let mut f = File::open(&path)?;
                    let mut buf = Vec::new();
                    f.read_to_end(&mut buf)?;
                    zip.write_all(&buf)?;
                }
            }
        }
        Ok(())
    }
}
