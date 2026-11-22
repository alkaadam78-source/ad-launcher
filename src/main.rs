use ad_launcher::app::{load_css, MainWindow, APP_ID};
use ad_launcher::core;
use gtk4::prelude::*;
use gtk4::Application;
use std::env;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--version" | "-v" => {
                println!(
                    "AD Launcher {} — The Dark Baroque Edition",
                    env!("CARGO_PKG_VERSION")
                );
                println!("Crafted by AdamDevnoob (Debian x86_64)");
                return;
            }
            "--help" | "-h" => {
                println!("AD Launcher Linux — The Dark Baroque Edition");
                println!("Usage: ad-launcher [OPTIONS] / nux-launcher [OPTIONS]");
                println!();
                println!("Options:");
                println!("  -v, --version    Show launcher version and credits");
                println!("  -h, --help       Show this help message");
                println!("  --launch         Launch the default Minecraft instance directly");
                return;
            }
            "--launch" => {
                println!("[AD LAUNCHER] Launching Minecraft with Adam Turbo Engine (Dark Baroque Edition)...");
                println!("[AD LAUNCHER] Crafted by AdamDevnoob");
                let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
                rt.block_on(async {
                    let mut core = match core::LauncherCore::init() {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!("Error initializing launcher: {}", e);
                            return;
                        }
                    };
                    let (tx, rx) = std::sync::mpsc::channel();
                    let handle = std::thread::spawn(move || {
                        while let Ok(event) = rx.recv() {
                            match event {
                                core::process::ProcessEvent::LogLine(line) => println!("{}", line),
                                core::process::ProcessEvent::Started(pid) => {
                                    println!("[NUX] Minecraft Process started with PID {}", pid)
                                }
                                core::process::ProcessEvent::Finished(code) => {
                                    println!(
                                        "[NUX] Minecraft Process finished with exit code {}",
                                        code
                                    );
                                    break;
                                }
                                core::process::ProcessEvent::Crashed(err) => {
                                    eprintln!("[NUX CRASH] {}", err);
                                    break;
                                }
                            }
                        }
                    });
                    if let Err(e) = core.launch_current(tx).await {
                        eprintln!("[NUX] Launch error: {}", e);
                    }
                    let _ = handle.join();
                });
                return;
            }
            _ => {}
        }
    }

    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    let _guard = rt.enter();

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| {
        load_css();
        gtk4::Window::set_default_icon_name("ad-launcher");
    });

    app.connect_activate(|app| {
        let win = MainWindow::new(app);
        win.window.present();
    });

    let empty_args: [&str; 0] = [];
    app.run_with_args(&empty_args);
}
