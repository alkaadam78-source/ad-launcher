use crate::core::manifest::{AssetIndex, DownloadArtifact, VersionPackage};
use crate::utils::crypto::verify_sha1;
use crate::utils::fs::ensure_dir;
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub const ASSET_BASE_URL: &str = "https://resources.download.minecraft.net";

#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub current_file: String,
    pub completed_items: usize,
    pub total_items: usize,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
}

pub struct MinecraftDownloader {
    client: Client,
    minecraft_dir: PathBuf,
}

impl MinecraftDownloader {
    pub fn new<P: AsRef<Path>>(minecraft_dir: P) -> Self {
        let client = Client::builder()
            .user_agent("NUX-Launcher-Linux/1.0.0 (Debian; Rust)")
            .build()
            .unwrap_or_default();

        Self {
            client,
            minecraft_dir: minecraft_dir.as_ref().to_path_buf(),
        }
    }

    /// Downloads a single file with optional SHA-1 verification
    pub async fn download_file(
        &self,
        url: &str,
        dest: &Path,
        expected_sha1: Option<&str>,
        expected_size: Option<u64>,
    ) -> Result<bool, String> {
        // Check if already valid
        if dest.exists() {
            if let Some(sha1) = expected_sha1 {
                if verify_sha1(dest, sha1) {
                    return Ok(false); // Skipped because already up to date
                }
            } else if let Some(size) = expected_size {
                if let Ok(meta) = std::fs::metadata(dest) {
                    if meta.len() == size {
                        return Ok(false);
                    }
                }
            }
        }

        if let Some(parent) = dest.parent() {
            ensure_dir(parent).map_err(|e| format!("Failed to create parent dir: {}", e))?;
        }

        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Failed to request {}: {}", url, e))?;

        if !resp.status().is_success() {
            return Err(format!("Download failed with status {} for {}", resp.status(), url));
        }

        let bytes = resp
            .bytes()
            .await
            .map_err(|e| format!("Failed to read body from {}: {}", url, e))?;

        // Write to temporary file first
        let tmp_dest = dest.with_extension("download.tmp");
        {
            let mut file = File::create(&tmp_dest)
                .map_err(|e| format!("Failed to create temp file: {}", e))?;
            file.write_all(&bytes)
                .map_err(|e| format!("Failed to write temp file: {}", e))?;
        }

        // Verify SHA1
        if let Some(sha1) = expected_sha1 {
            if !verify_sha1(&tmp_dest, sha1) {
                let _ = std::fs::remove_file(&tmp_dest);
                return Err(format!("Checksum mismatch for {}: expected {}", url, sha1));
            }
        }

        // Rename atomically
        std::fs::rename(&tmp_dest, dest)
            .map_err(|e| format!("Failed to finalize downloaded file: {}", e))?;

        Ok(true)
    }

    /// Downloads the version package JSON if not already present
    pub async fn download_version_package(&self, version_id: &str, url: &str) -> Result<PathBuf, String> {
        let dest = self
            .minecraft_dir
            .join("versions")
            .join(version_id)
            .join(format!("{}.json", version_id));

        self.download_file(url, &dest, None, None).await?;
        Ok(dest)
    }

    /// Downloads the client JAR (e.g. 1.20.1.jar)
    pub async fn download_client_jar(
        &self,
        version_id: &str,
        artifact: &DownloadArtifact,
    ) -> Result<PathBuf, String> {
        let dest = self
            .minecraft_dir
            .join("versions")
            .join(version_id)
            .join(format!("{}.jar", version_id));

        self.download_file(&artifact.url, &dest, Some(&artifact.sha1), Some(artifact.size))
            .await?;
        Ok(dest)
    }

    /// Downloads all required libraries for Linux x86_64
    pub async fn download_libraries(
        &self,
        pkg: &VersionPackage,
        on_progress: Option<Arc<dyn Fn(DownloadProgress) + Send + Sync>>,
    ) -> Result<Vec<PathBuf>, String> {
        let libs_dir = self.minecraft_dir.join("libraries");
        let mut applicable_libs = Vec::new();

        for lib in &pkg.libraries {
            if !lib.is_applicable_for_linux() {
                continue;
            }

            if let Some(downloads) = &lib.downloads {
                if let Some(art) = &downloads.artifact {
                    let rel_path = art
                        .path
                        .clone()
                        .or_else(|| lib.artifact_path())
                        .ok_or_else(|| format!("No path for library {}", lib.name))?;

                    let local_path = libs_dir.join(&rel_path);
                    applicable_libs.push((art.url.clone(), local_path, art.sha1.clone(), art.size));
                }
            }
        }

        let total_items = applicable_libs.len();
        let completed_items = Arc::new(AtomicU64::new(0));

        for (i, (url, dest, sha1, size)) in applicable_libs.iter().enumerate() {
            if let Some(ref cb) = on_progress {
                cb(DownloadProgress {
                    current_file: dest
                        .file_name()
                        .map(|f| f.to_string_lossy().to_string())
                        .unwrap_or_else(|| url.clone()),
                    completed_items: i,
                    total_items,
                    downloaded_bytes: completed_items.load(Ordering::Relaxed),
                    total_bytes: 0,
                });
            }

            self.download_file(url, dest, Some(sha1), Some(*size)).await?;
            completed_items.fetch_add(1, Ordering::Relaxed);
        }

        let result_paths = applicable_libs.into_iter().map(|(_, p, _, _)| p).collect();
        Ok(result_paths)
    }

    /// Downloads the asset index and all game asset objects
    pub async fn download_assets(
        &self,
        pkg: &VersionPackage,
        on_progress: Option<Arc<dyn Fn(DownloadProgress) + Send + Sync>>,
    ) -> Result<(), String> {
        let assets_dir = self.minecraft_dir.join("assets");
        let indexes_dir = assets_dir.join("indexes");
        let objects_dir = assets_dir.join("objects");

        ensure_dir(&indexes_dir).map_err(|e| e.to_string())?;
        ensure_dir(&objects_dir).map_err(|e| e.to_string())?;

        // 1. Download asset index JSON
        let index_dest = indexes_dir.join(format!("{}.json", pkg.asset_index.id));
        self.download_file(
            &pkg.asset_index.url,
            &index_dest,
            Some(&pkg.asset_index.sha1),
            Some(pkg.asset_index.size),
        )
        .await?;

        // 2. Parse index
        let content = std::fs::read_to_string(&index_dest)
            .map_err(|e| format!("Failed to read asset index: {}", e))?;
        let asset_index: AssetIndex = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse asset index JSON: {}", e))?;

        let total_items = asset_index.objects.len();
        let mut idx = 0;

        for (name, obj) in &asset_index.objects {
            let prefix = &obj.hash[..2];
            let obj_dest = objects_dir.join(prefix).join(&obj.hash);

            if !obj_dest.exists() || !verify_sha1(&obj_dest, &obj.hash) {
                let url = format!("{}/{}/{}", ASSET_BASE_URL, prefix, obj.hash);
                self.download_file(&url, &obj_dest, Some(&obj.hash), Some(obj.size)).await?;
            }

            idx += 1;
            if idx % 100 == 0 || idx == total_items {
                if let Some(ref cb) = on_progress {
                    cb(DownloadProgress {
                        current_file: name.clone(),
                        completed_items: idx,
                        total_items,
                        downloaded_bytes: idx as u64,
                        total_bytes: total_items as u64,
                    });
                }
            }
        }

        Ok(())
    }
}
