use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::thread;

#[derive(Debug, Clone)]
pub enum ProcessEvent {
    LogLine(String),
    Started(u32), // PID
    Finished(i32), // Exit code
    Crashed(String), // Crash report or error
}

pub struct LaunchConfig {
    pub java_binary: PathBuf,
    pub jvm_args: Vec<String>,
    pub classpath: Vec<PathBuf>,
    pub main_class: String,
    pub game_args: Vec<String>,
    pub game_dir: PathBuf,
}

pub struct MinecraftProcess;

impl MinecraftProcess {
    pub fn spawn(config: LaunchConfig, event_tx: Sender<ProcessEvent>) -> Result<u32, String> {
        let cp_separator = ":"; // Linux classpath separator
        let classpath_str = config
            .classpath
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(cp_separator);

        let mut cmd = Command::new(&config.java_binary);
        cmd.current_dir(&config.game_dir);

        // Hyper-Optimized Linux Mesa & OpenGL Driver Settings (Credit By AdamDevnoob)
        cmd.env("mesa_glthread", "true");
        cmd.env("vblank_mode", "0");
        cmd.env("MESA_GL_VERSION_OVERRIDE", "4.6");
        cmd.env("MESA_GLSL_VERSION_OVERRIDE", "460");
        cmd.env("allow_higher_compat_version", "true");
        cmd.env("INTEL_DEBUG", "norbc");

        // Forward display environment variables if set
        if let Ok(val) = std::env::var("DISPLAY") {
            cmd.env("DISPLAY", val);
        }
        if let Ok(val) = std::env::var("WAYLAND_DISPLAY") {
            cmd.env("WAYLAND_DISPLAY", val);
        }
        if let Ok(val) = std::env::var("XDG_RUNTIME_DIR") {
            cmd.env("XDG_RUNTIME_DIR", val);
        }

        // Add JVM arguments
        for arg in &config.jvm_args {
            cmd.arg(arg);
        }

        // Add Classpath
        cmd.arg("-cp").arg(&classpath_str);

        // Add Main Class
        cmd.arg(&config.main_class);

        // Add Game Arguments
        for arg in &config.game_args {
            cmd.arg(arg);
        }

        // Configure stdio pipes
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        log::info!("Spawning Minecraft via {:?}", config.java_binary);

        let mut child = cmd
            .spawn()
            .map_err(|e| format!("Failed to start Minecraft process: {}", e))?;

        let pid = child.id();
        let _ = event_tx.send(ProcessEvent::Started(pid));

        // Pipe stdout
        if let Some(stdout) = child.stdout.take() {
            let tx = event_tx.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines().flatten() {
                    let _ = tx.send(ProcessEvent::LogLine(line));
                }
            });
        }

        // Pipe stderr
        if let Some(stderr) = child.stderr.take() {
            let tx = event_tx.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines().flatten() {
                    let _ = tx.send(ProcessEvent::LogLine(format!("[STDERR] {}", line)));
                }
            });
        }

        // Monitor process exit in background thread
        let game_dir = config.game_dir.clone();
        thread::spawn(move || {
            match child.wait() {
                Ok(status) => {
                    let code = status.code().unwrap_or(-1);
                    if code != 0 {
                        // Check for crash reports in game_dir/crash-reports
                        let crash_msg = Self::find_latest_crash_report(&game_dir)
                            .unwrap_or_else(|| format!("Minecraft terminated with exit code {}", code));
                        let _ = event_tx.send(ProcessEvent::Crashed(crash_msg));
                    }
                    let _ = event_tx.send(ProcessEvent::Finished(code));
                }
                Err(e) => {
                    let _ = event_tx.send(ProcessEvent::Crashed(format!("Process monitor error: {}", e)));
                }
            }
        });

        Ok(pid)
    }

    fn find_latest_crash_report(game_dir: &Path) -> Option<String> {
        let crash_dir = game_dir.join("crash-reports");
        if !crash_dir.exists() {
            return None;
        }

        let mut entries: Vec<_> = std::fs::read_dir(crash_dir)
            .ok()?
            .flatten()
            .filter(|e| e.path().extension().map(|x| x == "txt").unwrap_or(false))
            .collect();

        entries.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());

        if let Some(latest) = entries.last() {
            if let Ok(content) = std::fs::read_to_string(latest.path()) {
                let preview: Vec<&str> = content.lines().take(40).collect();
                return Some(format!(
                    "Crash Report: {}\n\n{}",
                    latest.path().display(),
                    preview.join("\n")
                ));
            }
        }
        None
    }
}
