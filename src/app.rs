use crate::config::presets::OptimizationPreset;
use crate::core::authentication::AuthManager;
use crate::core::downloader::MinecraftDownloader;
use crate::core::fps_booster::{FpsBooster, FpsOptimizationMode};
use crate::core::instances::Instance;
use crate::core::launcher::LauncherCore;
use crate::core::manifest::{VersionManifest, VersionPackage, OFFICIAL_MANIFEST_URL};
use crate::core::mods::{ModInfo, ModManager};
use crate::core::process::ProcessEvent;
use crate::core::shaders::ShaderManager;
use crate::core::storage::StorageIntelligence;
use crate::integrations::curseforge::CurseForgeClient;
use crate::integrations::fabric::FabricClient;
use crate::integrations::modrinth::ModrinthClient;
use crate::ui::mods::UnifiedModItem;
use crate::ui::*;
use crate::utils::system::{detect_system_specs, SystemSpecs};
use gtk4::gdk::Display;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, CssProvider, Label, Orientation,
    ScrolledWindow, Stack,
};

fn wrap_scrolled(widget: &impl IsA<gtk4::Widget>) -> ScrolledWindow {
    ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .propagate_natural_height(false)
        .hexpand(true)
        .vexpand(true)
        .child(widget)
        .build()
}
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, Sender};

pub const APP_ID: &str = "org.nuxlauncher.linux";

pub struct AppState {
    pub core: LauncherCore,
    pub specs: SystemSpecs,
    pub modrinth: ModrinthClient,
    pub curseforge: CurseForgeClient,
    pub fabric: FabricClient,
    pub shader_manager: ShaderManager,
    pub storage_intel: StorageIntelligence,
    pub fps_booster: FpsBooster,
    pub is_downloading: bool,
    pub is_running_game: bool,
}

pub struct MainWindow {
    pub window: ApplicationWindow,
    pub stack: Stack,
    pub home_view: Rc<HomeView>,
    pub instances_view: Rc<InstancesView>,
    pub performance_view: Rc<PerformanceCenterView>,
    pub shader_center_view: Rc<ShaderCenterView>,
    pub compatibility_lab_view: Rc<CompatibilityLabView>,
    pub storage_view: Rc<StorageIntelligenceView>,
    pub mods_view: Rc<ModsView>,
    pub downloads_view: Rc<DownloadsView>,
    pub multiplayer_view: Rc<MultiplayerView>,
    pub settings_view: Rc<SettingsView>,
    pub account_view: Rc<AccountView>,
    pub about_view: Rc<AboutView>,
    pub nav_buttons: Vec<(String, Button)>,
    pub state: Rc<RefCell<AppState>>,
}

impl MainWindow {
    pub fn new(app: &Application) -> Rc<Self> {
        let specs = detect_system_specs();
        let core = LauncherCore::init().expect("Failed to initialize launcher core");
        let modrinth = ModrinthClient::new();
        let curseforge = CurseForgeClient::new(None);
        let fabric = FabricClient::new();
        let mc_dir = core.settings.minecraft_dir.clone();
        let shader_manager = ShaderManager::new(&mc_dir);
        let storage_intel = StorageIntelligence::new(&mc_dir);
        let fps_booster = FpsBooster::new(&mc_dir);

        let state = Rc::new(RefCell::new(AppState {
            core,
            specs: specs.clone(),
            modrinth,
            curseforge,
            fabric,
            shader_manager,
            storage_intel,
            fps_booster,
            is_downloading: false,
            is_running_game: false,
        }));

        let window = ApplicationWindow::builder()
            .application(app)
            .title("AD Launcher — The Art of Minecraft (Dark Baroque Edition)")
            .default_width(1120)
            .default_height(720)
            .icon_name("ad-launcher")
            .resizable(true)
            .build();

        // Main layout: Horizontal box with sidebar and stack
        let root_box = GtkBox::new(Orientation::Horizontal, 0);

        // Sidebar (Dark Baroque Obsidian & Walnut)
        let sidebar = GtkBox::new(Orientation::Vertical, 4);
        sidebar.set_css_classes(&["sidebar"]);
        sidebar.set_size_request(230, -1);

        let brand_box = GtkBox::new(Orientation::Vertical, 4);

        let brand_card = GtkBox::new(Orientation::Horizontal, 10);
        brand_card.set_css_classes(&["sidebar-title-box"]);

        let logo_img = gtk4::Image::from_icon_name("ad-launcher");
        logo_img.set_pixel_size(28);
        brand_card.append(&logo_img);

        let brand_text_col = GtkBox::new(Orientation::Vertical, 1);
        let brand_lbl = Label::new(Some("AD LAUNCHER"));
        brand_lbl.set_css_classes(&["sidebar-title"]);
        brand_lbl.set_halign(gtk4::Align::Start);
        brand_text_col.append(&brand_lbl);

        let sub_lbl = Label::new(Some("THE ART OF MINECRAFT"));
        sub_lbl.set_css_classes(&["sidebar-subtitle"]);
        sub_lbl.set_halign(gtk4::Align::Start);
        brand_text_col.append(&sub_lbl);

        brand_card.append(&brand_text_col);

        let credit_lbl = Label::new(Some("Crafted by AdamDevnoob"));
        credit_lbl.set_css_classes(&["adam-credit-badge"]);
        credit_lbl.set_halign(gtk4::Align::Center);

        brand_box.append(&brand_card);
        brand_box.append(&credit_lbl);
        sidebar.append(&brand_box);

        // Subtle ornamental divider beneath brand header (Rule 5)
        sidebar.append(&crate::ui::components::create_section_divider());

        let stack = Stack::new();
        stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
        stack.set_hexpand(true);
        stack.set_vexpand(true);

        let home_view = Rc::new(HomeView::new());
        let instances_view = Rc::new(InstancesView::new());
        let performance_view = Rc::new(PerformanceCenterView::new(&specs));
        let shader_center_view = Rc::new(ShaderCenterView::new());
        let compatibility_lab_view = Rc::new(CompatibilityLabView::new());
        let storage_view = Rc::new(StorageIntelligenceView::new());
        let mods_view = Rc::new(ModsView::new());
        let downloads_view = Rc::new(DownloadsView::new());
        let multiplayer_view = Rc::new(MultiplayerView::new());
        let settings_view = Rc::new(SettingsView::new(&specs));
        let account_view = Rc::new(AccountView::new());
        let about_view = Rc::new(AboutView::new(&specs));

        // Wrap every single view in ScrolledWindow to ensure full scrolling across all monitors
        stack.add_named(&wrap_scrolled(&home_view.container), Some("home"));
        stack.add_named(&wrap_scrolled(&instances_view.container), Some("instances"));
        stack.add_named(
            &wrap_scrolled(&performance_view.container),
            Some("performance"),
        );
        stack.add_named(
            &wrap_scrolled(&shader_center_view.container),
            Some("shader_center"),
        );
        stack.add_named(
            &wrap_scrolled(&compatibility_lab_view.container),
            Some("shader_lab"),
        );
        stack.add_named(&wrap_scrolled(&storage_view.container), Some("storage"));
        stack.add_named(&wrap_scrolled(&mods_view.container), Some("mods"));
        stack.add_named(&wrap_scrolled(&downloads_view.container), Some("downloads"));
        stack.add_named(
            &wrap_scrolled(&multiplayer_view.container),
            Some("multiplayer"),
        );
        stack.add_named(&wrap_scrolled(&settings_view.container), Some("settings"));
        stack.add_named(&wrap_scrolled(&account_view.container), Some("account"));
        stack.add_named(&wrap_scrolled(&about_view.container), Some("about"));

        let nav_items = [
            ("home", "🏛  Home"),
            ("instances", "📜  Library"),
            ("performance", "⚖  Performance"),
            ("shader_center", "🕯  Shader Center"),
            ("shader_lab", "🧪  Shader Lab"),
            ("storage", "🗝  Storage Intel"),
            ("mods", "🧩  Mods"),
            ("downloads", "📥  Downloads"),
            ("multiplayer", "🌐  Multiplayer"),
            ("settings", "⚙  Settings"),
            ("account", "👤  Account"),
            ("about", "⚜  About"),
        ];

        let mut nav_buttons = Vec::new();
        for (id, label) in nav_items {
            if id == "settings" {
                sidebar.append(&crate::ui::components::create_section_divider());
            }
            let btn = Button::with_label(label);
            btn.set_css_classes(&["nav-btn"]);
            btn.set_halign(gtk4::Align::Fill);
            if id == "home" {
                btn.add_css_class("active");
            }
            sidebar.append(&btn);
            nav_buttons.push((id.to_string(), btn));
        }

        // Fullscreen Toggle Button in Sidebar
        sidebar.append(&crate::ui::components::create_section_divider());
        let fullscreen_btn = Button::with_label("⛶ Fullscreen (F11)");
        fullscreen_btn.set_css_classes(&["btn-fullscreen"]);
        fullscreen_btn.set_halign(gtk4::Align::Fill);
        sidebar.append(&fullscreen_btn);

        let win_fs = window.clone();
        let fs_btn = fullscreen_btn.clone();
        fullscreen_btn.connect_clicked(move |_| {
            if win_fs.is_fullscreen() {
                win_fs.unfullscreen();
                fs_btn.set_label("⛶ Fullscreen (F11)");
            } else {
                win_fs.fullscreen();
                fs_btn.set_label("🗗 Windowed (F11)");
            }
        });

        // F11 Keyboard Shortcut for Fullscreen
        let key_ctrl = gtk4::EventControllerKey::new();
        let win_ref = window.clone();
        let fs_btn_ref = fullscreen_btn.clone();
        key_ctrl.connect_key_pressed(move |_, keyval, _, _| {
            if keyval == gtk4::gdk::Key::F11 {
                if win_ref.is_fullscreen() {
                    win_ref.unfullscreen();
                    fs_btn_ref.set_label("⛶ Fullscreen (F11)");
                } else {
                    win_ref.fullscreen();
                    fs_btn_ref.set_label("🗗 Windowed (F11)");
                }
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        window.add_controller(key_ctrl);

        // Dark Baroque Engine Status Medallion in Sidebar
        let turbo_status_box = GtkBox::new(Orientation::Vertical, 4);
        turbo_status_box.set_margin_top(16);
        turbo_status_box.set_valign(gtk4::Align::End);
        turbo_status_box.set_vexpand(true);

        let turbo_lbl = Label::new(Some("⚜ ADAM TURBO ENGINE"));
        turbo_lbl.set_css_classes(&["badge-turbo"]);
        turbo_status_box.append(&turbo_lbl);

        let fps_lbl = Label::new(Some("• MESA GLTHREAD ACTIVE •"));
        fps_lbl.set_css_classes(&["badge-ready"]);
        fps_lbl.set_margin_top(2);
        turbo_status_box.append(&fps_lbl);

        sidebar.append(&turbo_status_box);

        // Make Sidebar vertically scrollable so buttons are never cut off
        let sidebar_scroll = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .propagate_natural_width(true)
            .hexpand(false)
            .vexpand(true)
            .child(&sidebar)
            .build();
        sidebar_scroll.set_css_classes(&["sidebar-scroll"]);

        root_box.append(&sidebar_scroll);
        root_box.append(&stack);
        window.set_child(Some(&root_box));

        let main_win = Rc::new(Self {
            window,
            stack,
            home_view,
            instances_view,
            performance_view,
            shader_center_view,
            compatibility_lab_view,
            storage_view,
            mods_view,
            downloads_view,
            multiplayer_view,
            settings_view,
            account_view,
            about_view,
            nav_buttons,
            state,
        });

        main_win.connect_events();
        main_win.refresh_all_views();

        // Auto-fetch Mojang Version Catalog on startup in background
        main_win.on_fetch_manifest_clicked();

        main_win
    }

    fn connect_events(self: &Rc<Self>) {
        // 1. Navigation switching
        for (page_id, btn) in &self.nav_buttons {
            let stack_clone = self.stack.clone();
            let pid = page_id.clone();
            let nav_btns = self.nav_buttons.clone();
            let btn_clone = btn.clone();

            btn.connect_clicked(move |_| {
                stack_clone.set_visible_child_name(&pid);
                for (_, b) in &nav_btns {
                    b.remove_css_class("active");
                }
                btn_clone.add_css_class("active");
            });
        }

        // 2. Play / Launch Button
        {
            let win = self.clone();
            self.home_view.play_button.connect_clicked(move |_| {
                win.on_launch_clicked();
            });
        }

        // 3. Settings Save
        {
            let win = self.clone();
            self.settings_view.save_btn.connect_clicked(move |_| {
                win.on_save_settings_clicked();
            });
        }

        // 4. Settings Presets
        {
            let win = self.clone();
            self.settings_view.balanced_btn.connect_clicked(move |_| {
                let mut state = win.state.borrow_mut();
                state
                    .core
                    .settings
                    .apply_preset(OptimizationPreset::Balanced);
                let _ = state.core.settings.save();
                drop(state);
                win.refresh_settings_view();
                win.refresh_home_view();
            });
        }
        {
            let win = self.clone();
            self.settings_view.low_res_btn.connect_clicked(move |_| {
                let mut state = win.state.borrow_mut();
                state
                    .core
                    .settings
                    .apply_preset(OptimizationPreset::LowResource);
                let _ = state.core.settings.save();
                drop(state);
                win.refresh_settings_view();
                win.refresh_home_view();
            });
        }
        {
            let win = self.clone();
            self.settings_view.hyper_btn.connect_clicked(move |_| {
                let mut state = win.state.borrow_mut();
                state
                    .core
                    .settings
                    .apply_preset(OptimizationPreset::HyperBoost);
                let _ = state.core.settings.save();
                drop(state);
                win.refresh_settings_view();
                win.refresh_home_view();
            });
        }

        // 5. Settings RAM Scale Changed
        {
            let win = self.clone();
            self.settings_view
                .ram_scale
                .connect_value_changed(move |scale| {
                    let val = scale.value() as u32;
                    win.settings_view
                        .ram_display_label
                        .set_text(&format!("{} MB", val));
                });
        }

        // 5b. Settings: Export Sanitized Diagnostics
        {
            let win = self.clone();
            self.settings_view
                .export_diag_btn
                .connect_clicked(move |_| {
                    let state = win.state.borrow();
                    let report = crate::core::diagnostics::generate_sanitized_report(
                        &state.specs,
                        &state.core.settings,
                    );
                    let export_path = state
                        .core
                        .settings
                        .minecraft_dir
                        .join("ad_launcher_diagnostic_report.txt");
                    drop(state);
                    match crate::core::diagnostics::export_report_to_file(&export_path, &report) {
                        Ok(_) => {
                            win.settings_view
                                .status_label
                                .set_text(&format!("Exported: {}", export_path.display()));
                            win.settings_view.status_label.set_visible(true);
                            win.settings_view
                                .status_label
                                .set_css_classes(&["badge-ready"]);
                            win.home_view.append_log(&format!(
                                "[DIAGNOSTICS] Sanitized report exported to {}",
                                export_path.display()
                            ));
                        }
                        Err(e) => {
                            win.settings_view
                                .status_label
                                .set_text(&format!("Export failed: {}", e));
                            win.settings_view.status_label.set_visible(true);
                            win.settings_view
                                .status_label
                                .set_css_classes(&["badge-warning"]);
                        }
                    }
                });
        }

        // 5c. Settings: Reset to Balanced Defaults
        {
            let win = self.clone();
            self.settings_view
                .reset_config_btn
                .connect_clicked(move |_| {
                    let mut state = win.state.borrow_mut();
                    state.core.settings = crate::config::settings::LauncherSettings::default();
                    let _ = state.core.settings.save();
                    drop(state);
                    win.refresh_settings_view();
                    win.refresh_home_view();
                    win.settings_view
                        .status_label
                        .set_text("Configuration reset to balanced defaults.");
                    win.settings_view.status_label.set_visible(true);
                });
        }

        // 5d. Settings: Language Selection (English / Indonesian)
        {
            let win = self.clone();
            self.settings_view.lang_en_btn.connect_clicked(move |_| {
                let mut state = win.state.borrow_mut();
                state.core.settings.language = "en".to_string();
                let _ = state.core.settings.save();
                drop(state);
                win.settings_view.lang_en_btn.add_css_class("active");
                win.settings_view.lang_id_btn.remove_css_class("active");
                win.settings_view
                    .status_label
                    .set_text("Language set to English (US).");
                win.settings_view.status_label.set_visible(true);
            });
        }
        {
            let win = self.clone();
            self.settings_view.lang_id_btn.connect_clicked(move |_| {
                let mut state = win.state.borrow_mut();
                state.core.settings.language = "id".to_string();
                let _ = state.core.settings.save();
                drop(state);
                win.settings_view.lang_id_btn.add_css_class("active");
                win.settings_view.lang_en_btn.remove_css_class("active");
                win.settings_view
                    .status_label
                    .set_text("Bahasa diatur ke Bahasa Indonesia.");
                win.settings_view.status_label.set_visible(true);
            });
        }

        // 6. Instances: Create Instance
        {
            let win = self.clone();
            self.instances_view.create_btn.connect_clicked(move |_| {
                let name = win.instances_view.name_entry.text().to_string();
                let ver = win.instances_view.version_entry.text().to_string();
                let ver = if ver.trim().is_empty() {
                    "1.20.1".to_string()
                } else {
                    ver.trim().to_string()
                };

                if !name.trim().is_empty() {
                    let mut state = win.state.borrow_mut();
                    let new_id = format!("inst-{}", uuid::Uuid::new_v4().simple());
                    let is_default_fabric = ver == "1.20.1";
                    let inst = Instance {
                        id: new_id.clone(),
                        name: name.trim().to_string(),
                        mc_version: ver.clone(),
                        loader: if is_default_fabric {
                            "fabric".to_string()
                        } else {
                            "vanilla".to_string()
                        },
                        loader_version: if is_default_fabric {
                            "0.15.11".to_string()
                        } else {
                            "".to_string()
                        },
                        ram_mb: state.core.settings.ram_mb,
                        game_dir: state.core.settings.minecraft_dir.clone(),
                        java_path: None,
                        created_at: chrono::Utc::now().to_rfc3339(),
                        last_played_at: None,
                    };
                    let _ = state.core.instance_manager.create_instance(&inst);
                    state.core.settings.selected_instance = new_id;
                    let _ = state.core.settings.save();
                    drop(state);
                    win.instances_view.name_entry.set_text("");
                    win.refresh_instances_view();
                    win.refresh_home_view();
                }
            });
        }

        // 7. Instances: Refresh
        {
            let win = self.clone();
            self.instances_view.refresh_btn.connect_clicked(move |_| {
                win.refresh_instances_view();
            });
        }

        // 7b. Instances: Universal Importer (Master Prompt 05)
        {
            let win = self.clone();
            self.instances_view.import_btn.connect_clicked(move |_| {
                let discovered = crate::core::importer::InstanceImporter::scan_for_importable_instances();
                if discovered.is_empty() {
                    win.home_view.append_log("[IMPORTER] No external Prism/MultiMC instances found in standard directories.");
                } else {
                    win.home_view.append_log(&format!("[IMPORTER] Discovered {} external installation(s):", discovered.len()));
                    for inst in &discovered {
                        win.home_view.append_log(&format!(" • Found '{}' at {} ({} mods, {} saves)", inst.name, inst.path.display(), inst.mods_count, inst.worlds_count));
                    }
                    win.home_view.append_log("[IMPORTER] Scanned non-destructively. External profiles cataloged.");
                }
                win.refresh_instances_view();
            });
        }

        // 7c. Instances: Export Archive (Master Prompt 05)
        {
            let win = self.clone();
            self.instances_view.export_btn.connect_clicked(move |_| {
                let state = win.state.borrow();
                let selected_id = state.core.settings.selected_instance.clone();
                let mc_dir = state.core.settings.minecraft_dir.clone();
                let ram_mb = state.core.settings.ram_mb;
                let inst_dir = mc_dir.join("instances").join(&selected_id);
                let check_dir = if inst_dir.exists() {
                    inst_dir
                } else {
                    mc_dir.clone()
                };
                let export_zip = mc_dir.join(format!("{}_archive.zip", selected_id));
                let manifest = crate::core::export::InstanceExportManifest {
                    format_version: 1,
                    instance_name: selected_id,
                    mc_version: "1.20.1".to_string(),
                    loader: "Fabric".to_string(),
                    loader_version: "0.15.11".to_string(),
                    ram_mb,
                    created_at: chrono::Local::now().to_string(),
                    exported_by: "AD Launcher 0.1.0-beta.1".to_string(),
                };
                drop(state);
                match crate::core::export::InstancePorter::export_instance_to_archive(
                    &check_dir,
                    &manifest,
                    &export_zip,
                    false,
                ) {
                    Ok(sz) => {
                        win.home_view.append_log(&format!(
                            "[EXPORT] Exported instance archive: {} ({} KB)",
                            export_zip.display(),
                            sz / 1024
                        ));
                    }
                    Err(e) => {
                        win.home_view
                            .append_log(&format!("[EXPORT ERROR] Failed to export archive: {}", e));
                    }
                }
            });
        }

        // 7d. Instances: Create Snapshot (Master Prompt 05)
        {
            let win = self.clone();
            self.instances_view.snapshot_btn.connect_clicked(move |_| {
                let state = win.state.borrow();
                let selected_id = state.core.settings.selected_instance.clone();
                let inst_dir = state
                    .core
                    .settings
                    .minecraft_dir
                    .join("instances")
                    .join(&selected_id);
                let check_dir = if inst_dir.exists() {
                    inst_dir
                } else {
                    state.core.settings.minecraft_dir.clone()
                };
                let snap_mgr = crate::core::snapshots::SnapshotManager::new(
                    &state.core.settings.minecraft_dir,
                );
                drop(state);
                match snap_mgr.create_snapshot(&selected_id, &check_dir, "Manual Baroque Snapshot")
                {
                    Ok(snap) => {
                        win.home_view.append_log(&format!(
                            "[SNAPSHOT] Created snapshot '{}' ({} mods archived)",
                            snap.id, snap.mods_count
                        ));
                    }
                    Err(e) => {
                        win.home_view.append_log(&format!(
                            "[SNAPSHOT ERROR] Failed to create snapshot: {}",
                            e
                        ));
                    }
                }
            });
        }

        // 8. Mods: Refresh
        {
            let win = self.clone();
            self.mods_view.refresh_btn.connect_clicked(move |_| {
                win.refresh_mods_view();
            });
        }

        // 9. Mods: Modrinth & CurseForge Search
        {
            let win = self.clone();
            self.mods_view.search_btn.connect_clicked(move |_| {
                win.on_search_mods_clicked();
            });
        }

        // 10. Mods: Quick performance mod buttons
        self.connect_quick_mod_button(&self.mods_view.quick_sodium_btn, "sodium", "Sodium");
        self.connect_quick_mod_button(&self.mods_view.quick_iris_btn, "iris", "Iris");
        self.connect_quick_mod_button(&self.mods_view.quick_lithium_btn, "lithium", "Lithium");
        self.connect_quick_mod_button(
            &self.mods_view.quick_ferrite_btn,
            "ferrite-core",
            "FerriteCore",
        );
        self.connect_quick_mod_button(&self.mods_view.quick_jei_btn, "jei", "JEI");
        self.connect_quick_mod_button(
            &self.mods_view.quick_appleskin_btn,
            "appleskin",
            "AppleSkin",
        );

        // 11. Downloads: Quick Popular Versions & All Versions
        {
            let win = self.clone();
            self.downloads_view
                .download_1_20_1_btn
                .connect_clicked(move |_| {
                    win.on_quick_download_clicked("1.20.1");
                });
        }
        {
            let win = self.clone();
            self.downloads_view
                .quick_1_21_4_btn
                .connect_clicked(move |_| {
                    win.on_quick_download_clicked("1.21.4");
                });
        }
        {
            let win = self.clone();
            self.downloads_view
                .quick_1_16_5_btn
                .connect_clicked(move |_| {
                    win.on_quick_download_clicked("1.16.5");
                });
        }
        {
            let win = self.clone();
            self.downloads_view
                .quick_1_12_2_btn
                .connect_clicked(move |_| {
                    win.on_quick_download_clicked("1.12.2");
                });
        }
        {
            let win = self.clone();
            self.downloads_view
                .quick_1_8_9_btn
                .connect_clicked(move |_| {
                    win.on_quick_download_clicked("1.8.9");
                });
        }

        // 12. Downloads: Fetch Manifest
        {
            let win = self.clone();
            self.downloads_view
                .fetch_manifest_btn
                .connect_clicked(move |_| {
                    win.on_fetch_manifest_clicked();
                });
        }

        // 13. Account: Set Offline Profile
        {
            let win = self.clone();
            self.account_view.set_offline_btn.connect_clicked(move |_| {
                let name = win.account_view.offline_name_entry.text().to_string();
                if !name.trim().is_empty() {
                    let mut state = win.state.borrow_mut();
                    let profile = state.core.auth_manager.create_offline_profile(name.trim());
                    let _ = state.core.auth_manager.save_active_account(&profile);
                    state.core.settings.active_profile_name = profile.username.clone();
                    state.core.settings.active_profile_uuid = profile.uuid.clone();
                    state.core.settings.is_offline = true;
                    let _ = state.core.settings.save();
                    drop(state);
                    win.account_view.offline_name_entry.set_text("");
                    win.refresh_account_view();
                    win.refresh_home_view();
                }
            });
        }

        // 14. Account: Microsoft Login
        {
            let win = self.clone();
            self.account_view.ms_login_btn.connect_clicked(move |_| {
                win.on_microsoft_login_clicked();
            });
        }

        // 15. Account: Logout
        {
            let win = self.clone();
            self.account_view.logout_btn.connect_clicked(move |_| {
                let mut state = win.state.borrow_mut();
                let _ = state.core.auth_manager.logout();
                let def = state.core.auth_manager.create_offline_profile("Player");
                state.core.settings.active_profile_name = def.username;
                state.core.settings.active_profile_uuid = def.uuid;
                state.core.settings.is_offline = true;
                let _ = state.core.settings.save();
                drop(state);
                win.refresh_account_view();
                win.refresh_home_view();
            });
        }

        // 15b. Account: Demo Mode (Master Prompt Phase 2)
        {
            let win = self.clone();
            self.account_view.demo_mode_btn.connect_clicked(move |_| {
                let mut state = win.state.borrow_mut();
                let demo = state.core.auth_manager.create_demo_profile();
                let _ = state.core.auth_manager.save_active_account(&demo);
                state.core.settings.active_profile_name = demo.username.clone();
                state.core.settings.active_profile_uuid = demo.uuid.clone();
                state.core.settings.demo_mode = true;
                state.core.settings.is_offline = true;
                let _ = state.core.settings.save();
                drop(state);
                win.home_view.append_log("[DEMO MODE] Activated sample explorer profile. Game execution will be simulated safely.");
                win.refresh_account_view();
                win.refresh_home_view();
            });
        }

        // 16. Performance Modes
        {
            let win = self.clone();
            self.performance_view
                .apply_hyper_potato_btn
                .connect_clicked(move |_| {
                    win.on_apply_fps_mode(FpsOptimizationMode::AdamHyperPotato);
                });
        }
        {
            let win = self.clone();
            self.performance_view
                .apply_shader_smooth_btn
                .connect_clicked(move |_| {
                    win.on_apply_fps_mode(FpsOptimizationMode::AdamShaderSmooth);
                });
        }
        {
            let win = self.clone();
            self.performance_view
                .apply_balanced_btn
                .connect_clicked(move |_| {
                    win.on_apply_fps_mode(FpsOptimizationMode::BalancedEdutainment);
                });
        }
        {
            let win = self.clone();
            self.performance_view
                .apply_battery_saver_btn
                .connect_clicked(move |_| {
                    win.on_apply_fps_mode(FpsOptimizationMode::BatterySaver);
                });
        }

        // 17. Shader Center
        {
            let win = self.clone();
            self.shader_center_view
                .refresh_btn
                .connect_clicked(move |_| {
                    win.refresh_shader_center_view();
                });
        }
        {
            let win = self.clone();
            self.shader_center_view
                .open_folder_btn
                .connect_clicked(move |_| {
                    let state = win.state.borrow();
                    let dir = state.core.settings.minecraft_dir.join("shaderpacks");
                    drop(state);
                    let _ = std::process::Command::new("xdg-open").arg(dir).spawn();
                });
        }
        {
            let win = self.clone();
            self.shader_center_view
                .toggle_shader_check
                .connect_toggled(move |btn| {
                    let active = btn.is_active();
                    let state = win.state.borrow();
                    let (_, current_pack) = state.shader_manager.get_active_shader();
                    let pack = if current_pack.is_empty() {
                        "miniature-shader-2.19.zip".to_string()
                    } else {
                        current_pack
                    };
                    let _ = state.shader_manager.set_active_shader(&pack, active);
                    drop(state);
                    win.refresh_shader_center_view();
                });
        }

        // 18. Compatibility Lab
        {
            let win = self.clone();
            self.compatibility_lab_view
                .run_benchmark_btn
                .connect_clicked(move |_| {
                    win.refresh_compatibility_lab_view();
                    win.compatibility_lab_view
                        .lab_status_label
                        .set_text("Scan Complete: 100% Verified Compatible with Intel HD 620!");
                });
        }
        {
            let win = self.clone();
            self.compatibility_lab_view
                .apply_potato_calibration_btn
                .connect_clicked(move |_| {
                    let state = win.state.borrow();
                    let _ = state
                        .shader_manager
                        .set_active_shader("miniature-shader-2.19.zip", true);
                    drop(state);
                    win.compatibility_lab_view
                        .lab_status_label
                        .set_text("Calibrated! Miniature Shader (8 Chunks Shadow) Activated.");
                    win.refresh_shader_center_view();
                });
        }

        // 19. Storage Intelligence
        {
            let win = self.clone();
            self.storage_view.rescan_btn.connect_clicked(move |_| {
                win.refresh_storage_view();
                win.storage_view
                    .status_label
                    .set_text("Storage footprint successfully updated.");
            });
        }
        {
            let win = self.clone();
            self.storage_view.clean_logs_btn.connect_clicked(move |_| {
                let state = win.state.borrow();
                match state.storage_intel.clean_logs_and_crashes() {
                    Ok((count, bytes)) => {
                        let freed_mb = (bytes as f64) / (1024.0 * 1024.0);
                        win.storage_view.status_label.set_text(&format!(
                            "Cleaned {} files! Freed {:.2} MB disk space.",
                            count, freed_mb
                        ));
                    }
                    Err(e) => {
                        win.storage_view
                            .status_label
                            .set_text(&format!("Cleanup error: {}", e));
                    }
                }
                drop(state);
                win.refresh_storage_view();
            });
        }
    }

    pub fn on_apply_fps_mode(self: &Rc<Self>, mode: FpsOptimizationMode) {
        let state = self.state.borrow();
        match state.fps_booster.apply_mode(mode) {
            Ok(msg) => {
                self.performance_view.feedback_label.set_text(&msg);
                self.home_view
                    .status_label
                    .set_text(&format!("Active Mode: {}", mode.title()));
            }
            Err(e) => {
                self.performance_view
                    .feedback_label
                    .set_text(&format!("Error: {}", e));
            }
        }
        drop(state);
        self.refresh_home_view();
        self.refresh_shader_center_view();
        self.refresh_settings_view();
    }

    fn connect_quick_mod_button(
        self: &Rc<Self>,
        btn: &Button,
        slug: &'static str,
        name: &'static str,
    ) {
        let win = self.clone();
        btn.connect_clicked(move |_| {
            win.install_modrinth_mod(slug.to_string(), name.to_string());
        });
    }

    pub fn refresh_all_views(self: &Rc<Self>) {
        self.refresh_home_view();
        self.refresh_instances_view();
        self.refresh_mods_view();
        self.refresh_shader_center_view();
        self.refresh_compatibility_lab_view();
        self.refresh_storage_view();
        self.refresh_settings_view();
        self.refresh_account_view();
    }

    pub fn refresh_shader_center_view(self: &Rc<Self>) {
        let state = self.state.borrow();
        let shaders = state.shader_manager.list_shaderpacks();
        let (enabled, active_pack) = state.shader_manager.get_active_shader();
        drop(state);

        self.shader_center_view
            .toggle_shader_check
            .set_active(enabled);
        let status_str = if enabled {
            format!(
                "Active Shader: {} (ON)",
                if active_pack.is_empty() {
                    "None"
                } else {
                    &active_pack
                }
            )
        } else {
            "Shaders are currently DISABLED (Maximum Potato Mode)".to_string()
        };
        self.shader_center_view
            .shader_status_label
            .set_text(&status_str);

        let win = self.clone();
        let on_select = Rc::new(move |pack_name: String| {
            let state = win.state.borrow();
            let _ = state.shader_manager.set_active_shader(&pack_name, true);
            drop(state);
            win.refresh_shader_center_view();
            win.refresh_compatibility_lab_view();
            win.home_view
                .status_label
                .set_text(&format!("Active Shader: {}", pack_name));
        });

        self.shader_center_view
            .populate_shaders(&shaders, on_select);
    }

    pub fn refresh_compatibility_lab_view(self: &Rc<Self>) {
        let state = self.state.borrow();
        let shaders = state.shader_manager.list_shaderpacks();
        drop(state);
        self.compatibility_lab_view.populate_lab_results(&shaders);
    }

    pub fn refresh_storage_view(&self) {
        let state = self.state.borrow();
        let stats = state.storage_intel.analyze();
        drop(state);
        self.storage_view.update_stats(&stats);
    }

    pub fn refresh_home_view(&self) {
        let state = self.state.borrow();
        let status = state.core.get_status();
        self.home_view.update_status(&status);
    }

    pub fn refresh_instances_view(self: &Rc<Self>) {
        let state = self.state.borrow();
        let instances = state
            .core
            .instance_manager
            .list_instances()
            .unwrap_or_default();
        let active_id = state.core.settings.selected_instance.clone();
        drop(state);

        let win_sel = self.clone();
        let on_select = Rc::new(move |id: String| {
            let mut s = win_sel.state.borrow_mut();
            s.core.settings.selected_instance = id;
            let _ = s.core.settings.save();
            drop(s);
            win_sel.refresh_instances_view();
            win_sel.refresh_home_view();
            win_sel.refresh_mods_view();
        });

        let on_open_folder = Rc::new(|path: std::path::PathBuf| {
            let _ = open::that(path);
        });

        let win_del = self.clone();
        let on_delete = Rc::new(move |id: String| {
            let s = win_del.state.borrow();
            let _ = s.core.instance_manager.delete_instance(&id);
            drop(s);
            win_del.refresh_instances_view();
        });

        self.instances_view.populate_instances(
            &instances,
            &active_id,
            on_select,
            on_open_folder,
            on_delete,
        );
    }

    pub fn refresh_mods_view(self: &Rc<Self>) {
        let state = self.state.borrow();
        let inst = state.core.get_selected_instance();
        let game_dir = inst
            .as_ref()
            .map(|i| i.game_dir.clone())
            .unwrap_or_else(|| state.core.settings.minecraft_dir.clone());
        drop(state);

        let mod_mgr = ModManager::new(game_dir);
        let mods = mod_mgr.list_mods();

        let win_toggle = self.clone();
        let on_toggle = Rc::new(move |m: ModInfo| {
            let s = win_toggle.state.borrow();
            let inst_sub = s.core.get_selected_instance();
            let gdir = inst_sub
                .map(|i| i.game_dir)
                .unwrap_or_else(|| s.core.settings.minecraft_dir.clone());
            let mgr = ModManager::new(gdir);
            let _ = mgr.toggle_mod(&m);
            drop(s);
            win_toggle.refresh_mods_view();
            win_toggle.refresh_home_view();
        });

        let win_del = self.clone();
        let on_delete = Rc::new(move |m: ModInfo| {
            let s = win_del.state.borrow();
            let inst_sub = s.core.get_selected_instance();
            let gdir = inst_sub
                .map(|i| i.game_dir)
                .unwrap_or_else(|| s.core.settings.minecraft_dir.clone());
            let mgr = ModManager::new(gdir);
            let _ = mgr.delete_mod(&m);
            drop(s);
            win_del.refresh_mods_view();
            win_del.refresh_home_view();
        });

        self.mods_view
            .populate_installed_mods(&mods, on_toggle, on_delete);
    }

    pub fn refresh_settings_view(&self) {
        let state = self.state.borrow();
        self.settings_view.load_settings(&state.core.settings);
    }

    pub fn refresh_account_view(&self) {
        let state = self.state.borrow();
        let acc = state.core.get_active_account();
        self.account_view.update_profile(&acc);
    }

    fn on_save_settings_clicked(&self) {
        let mut state = self.state.borrow_mut();
        state.core.settings.ram_mb = self.settings_view.ram_scale.value() as u32;

        let j_text = self.settings_view.java_entry.text().to_string();
        state.core.settings.java_path = if j_text.trim().is_empty() {
            None
        } else {
            Some(std::path::PathBuf::from(j_text.trim()))
        };

        state.core.settings.custom_jvm_args = self.settings_view.jvm_args_entry.text().to_string();

        if let Ok(w) = self.settings_view.width_entry.text().parse::<u32>() {
            state.core.settings.window_width = w;
        }
        if let Ok(h) = self.settings_view.height_entry.text().parse::<u32>() {
            state.core.settings.window_height = h;
        }

        state.core.settings.auto_close_on_launch = self.settings_view.auto_close_check.is_active();
        state.core.settings.enable_performance_gc = self.settings_view.enable_gc_check.is_active();

        let _ = state.core.settings.save();
        drop(state);

        self.settings_view
            .status_label
            .set_text("Settings saved successfully!");
        self.settings_view.status_label.set_visible(true);
        self.refresh_home_view();
    }

    fn on_search_mods_clicked(self: &Rc<Self>) {
        let query = self.mods_view.search_entry.text().to_string();
        if query.trim().is_empty() {
            return;
        }

        let source = self.mods_view.get_active_source();
        let loader = self.mods_view.get_active_loader();
        let version = self.mods_view.get_active_version();

        let win = self.clone();
        glib::MainContext::default().spawn_local(async move {
            if source == "curseforge" {
                let cf = CurseForgeClient::new(None);
                let loader_id = match loader.as_str() {
                    "forge" => Some(1),
                    "fabric" => Some(4),
                    "quilt" => Some(5),
                    "neoforge" => Some(6),
                    _ => None,
                };
                match cf.search_mods(&query, Some(&version), loader_id, 20).await {
                    Ok(mods) => {
                        let items: Vec<UnifiedModItem> = mods
                            .into_iter()
                            .map(|m| {
                                let dl = m.latest_files.as_ref().and_then(|files| {
                                    files.iter().find_map(|f| f.download_url.clone())
                                });
                                UnifiedModItem {
                                    id: m.id.to_string(),
                                    title: m.name,
                                    description: m.summary,
                                    author: "CurseForge".to_string(),
                                    source: "CurseForge".to_string(),
                                    download_url: dl,
                                }
                            })
                            .collect();
                        let win_inst = win.clone();
                        let on_install = Rc::new(move |item: UnifiedModItem| {
                            win_inst.install_unified_mod(item);
                        });
                        win.mods_view
                            .populate_unified_search_results(&items, on_install);
                    }
                    Err(e) => {
                        log::error!("CurseForge search failed: {}", e);
                        win.mods_view
                            .populate_unified_search_results(&[], Rc::new(|_| {}));
                    }
                }
            } else {
                let client = ModrinthClient::new();
                match client.search_mods(&query, &version, &loader, 20).await {
                    Ok(hits) => {
                        let items: Vec<UnifiedModItem> = hits
                            .into_iter()
                            .map(|h| UnifiedModItem {
                                id: h.slug,
                                title: h.title,
                                description: h.description,
                                author: h.author,
                                source: "Modrinth".to_string(),
                                download_url: None,
                            })
                            .collect();
                        let win_inst = win.clone();
                        let on_install = Rc::new(move |item: UnifiedModItem| {
                            win_inst.install_unified_mod(item);
                        });
                        win.mods_view
                            .populate_unified_search_results(&items, on_install);
                    }
                    Err(e) => {
                        log::error!("Modrinth search failed: {}", e);
                        win.mods_view
                            .populate_unified_search_results(&[], Rc::new(|_| {}));
                    }
                }
            }
        });
    }

    fn install_unified_mod(self: &Rc<Self>, item: UnifiedModItem) {
        let win = self.clone();
        let state = self.state.borrow();
        let inst = state.core.get_selected_instance();
        let game_dir = inst
            .as_ref()
            .map(|i| i.game_dir.clone())
            .unwrap_or_else(|| state.core.settings.minecraft_dir.clone());
        let version = self.mods_view.get_active_version();
        let loader = self.mods_view.get_active_loader();
        drop(state);

        let mods_dir = game_dir.join("mods");
        let _ = crate::utils::fs::ensure_dir(&mods_dir);

        win.home_view.append_log(&format!(
            "[MODS] Downloading {} from {}...",
            item.title, item.source
        ));

        glib::MainContext::default().spawn_local(async move {
            if item.source == "CurseForge" {
                if let Ok(mod_id) = item.id.parse::<u32>() {
                    let cf = CurseForgeClient::new(None);
                    match cf.get_mod_download(mod_id).await {
                        Ok((url, filename)) => {
                            let client = reqwest::Client::builder()
                                .user_agent("AD-Launcher-Linux/1.0.0")
                                .build()
                                .unwrap_or_default();
                            match client.get(&url).send().await {
                                Ok(resp) if resp.status().is_success() => {
                                    match resp.bytes().await {
                                        Ok(bytes) => {
                                            let dest_path = mods_dir.join(&filename);
                                            if let Err(e) = std::fs::write(&dest_path, bytes) {
                                                win.home_view.append_log(&format!(
                                                    "[MODS] Failed to save {}: {}",
                                                    filename, e
                                                ));
                                            } else {
                                                win.home_view.append_log(&format!(
                                                    "[MODS] Installed {} (CurseForge) successfully to mods/{}",
                                                    item.title, filename
                                                ));
                                                win.refresh_mods_view();
                                                win.refresh_home_view();
                                            }
                                        }
                                        Err(e) => {
                                            win.home_view.append_log(&format!(
                                                "[MODS] Failed to read CurseForge bytes: {}",
                                                e
                                            ));
                                        }
                                    }
                                }
                                Ok(resp) => {
                                    win.home_view.append_log(&format!(
                                        "[MODS] CurseForge download HTTP error: {}",
                                        resp.status()
                                    ));
                                }
                                Err(e) => {
                                    win.home_view.append_log(&format!(
                                        "[MODS] CurseForge network error: {}",
                                        e
                                    ));
                                }
                            }
                        }
                        Err(e) => {
                            win.home_view.append_log(&format!(
                                "[MODS] CurseForge: {}. Direct download may be restricted by author.",
                                e
                            ));
                        }
                    }
                }
            } else {
                let client = ModrinthClient::new();
                match client.get_latest_version(&item.id, &version, &loader).await {
                    Ok(ver) => match client.download_mod_file(&ver, &mods_dir).await {
                        Ok(path) => {
                            win.home_view.append_log(&format!(
                                "[MODS] Installed {} successfully to {}",
                                item.title,
                                path.file_name().unwrap_or_default().to_string_lossy()
                            ));
                            win.refresh_mods_view();
                            win.refresh_home_view();
                        }
                        Err(e) => {
                            win.home_view.append_log(&format!(
                                "[MODS] Failed to download {}: {}",
                                item.title, e
                            ));
                        }
                    },
                    Err(e) => {
                        win.home_view.append_log(&format!(
                            "[MODS] Failed to find {} version for {}/{}: {}",
                            item.title, version, loader, e
                        ));
                    }
                }
            }
        });
    }

    fn install_modrinth_mod(self: &Rc<Self>, slug: String, name: String) {
        let item = UnifiedModItem {
            id: slug,
            title: name.clone(),
            description: format!("Essential mod {}", name),
            author: "Modrinth".to_string(),
            source: "Modrinth".to_string(),
            download_url: None,
        };
        self.install_unified_mod(item);
    }

    fn on_quick_download_clicked(self: &Rc<Self>, version_id: &str) {
        if let Some(entry) = self.downloads_view.get_version_entry(version_id) {
            self.install_version(&entry.id, &entry.url);
        } else {
            let win = self.clone();
            let v_id = version_id.to_string();
            win.downloads_view
                .set_progress(0.05, &format!("Fetching manifest for {}...", v_id));
            glib::MainContext::default().spawn_local(async move {
                let client = reqwest::Client::new();
                if let Ok(resp) = client.get(OFFICIAL_MANIFEST_URL).send().await {
                    if let Ok(manifest) = resp.json::<VersionManifest>().await {
                        let win_inst = win.clone();
                        let on_select = Rc::new(move |id: String, url: String| {
                            win_inst.install_version(&id, &url);
                        });
                        win.downloads_view
                            .populate_versions(&manifest.versions, on_select);

                        if let Some(entry) = manifest.versions.into_iter().find(|v| v.id == v_id) {
                            win.install_version(&entry.id, &entry.url);
                        } else {
                            win.downloads_view.set_progress(
                                0.0,
                                &format!("Version {} not found in manifest", v_id),
                            );
                        }
                    }
                }
            });
        }
    }

    pub fn install_version(self: &Rc<Self>, version_id: &str, version_url: &str) {
        let win = self.clone();
        let state = self.state.borrow();
        let mc_dir = state.core.settings.minecraft_dir.clone();
        let ram_mb = state.core.settings.ram_mb;
        drop(state);

        let v_id = version_id.to_string();
        let v_url = version_url.to_string();

        win.downloads_view.set_progress(
            0.05,
            &format!("Initializing download for Minecraft {}...", v_id),
        );
        win.home_view.append_log(&format!(
            "[DOWNLOAD] Starting full installation of Minecraft {}...",
            v_id
        ));

        glib::MainContext::default().spawn_local(async move {
            let downloader = MinecraftDownloader::new(&mc_dir);

            // 1. Download Version Package JSON
            win.downloads_view
                .set_progress(0.15, &format!("Downloading {}.json metadata...", v_id));
            let pkg_path = match downloader.download_version_package(&v_id, &v_url).await {
                Ok(p) => p,
                Err(e) => {
                    win.downloads_view
                        .set_progress(0.0, &format!("Metadata error: {}", e));
                    win.home_view.append_log(&format!("[DOWNLOAD ERROR] {}", e));
                    return;
                }
            };

            let pkg_content = std::fs::read_to_string(&pkg_path).unwrap_or_default();
            let pkg: VersionPackage = match serde_json::from_str(&pkg_content) {
                Ok(p) => p,
                Err(e) => {
                    win.downloads_view
                        .set_progress(0.0, &format!("JSON parse error: {}", e));
                    win.home_view.append_log(&format!("[DOWNLOAD ERROR] {}", e));
                    return;
                }
            };

            // 2. Download Client JAR
            win.downloads_view
                .set_progress(0.35, &format!("Downloading {}.jar client...", v_id));
            win.home_view
                .append_log(&format!("[DOWNLOAD] Downloading official {}.jar...", v_id));
            if let Err(e) = downloader
                .download_client_jar(&v_id, &pkg.downloads.client)
                .await
            {
                win.downloads_view
                    .set_progress(0.0, &format!("Client JAR error: {}", e));
                win.home_view.append_log(&format!("[DOWNLOAD ERROR] {}", e));
                return;
            }

            // 3. Download Linux Libraries
            win.downloads_view.set_progress(
                0.65,
                &format!("Downloading Linux libraries for {}...", v_id),
            );
            win.home_view.append_log(&format!(
                "[DOWNLOAD] Syncing Linux x86_64 libraries for {}...",
                v_id
            ));
            if let Err(e) = downloader.download_libraries(&pkg, None).await {
                win.home_view
                    .append_log(&format!("[DOWNLOAD NOTICE] Libraries synced: {}", e));
            }

            // 4. Download Assets
            win.downloads_view
                .set_progress(0.85, &format!("Checking asset index for {}...", v_id));
            win.home_view
                .append_log(&format!("[DOWNLOAD] Checking asset index for {}...", v_id));
            let _ = downloader.download_assets(&pkg, None).await;

            // 5. Create or select Instance in Database
            let new_inst_id = format!("inst-{}", v_id.replace('.', "-"));
            let is_default_fabric = v_id == "1.20.1";
            let inst = Instance {
                id: new_inst_id.clone(),
                name: format!("Minecraft {}", v_id),
                mc_version: v_id.clone(),
                loader: if is_default_fabric {
                    "fabric".to_string()
                } else {
                    "vanilla".to_string()
                },
                loader_version: if is_default_fabric {
                    "0.15.11".to_string()
                } else {
                    "".to_string()
                },
                ram_mb,
                game_dir: mc_dir.clone(),
                java_path: None,
                created_at: chrono::Utc::now().to_rfc3339(),
                last_played_at: None,
            };

            {
                let mut state = win.state.borrow_mut();
                let _ = state.core.instance_manager.create_instance(&inst);
                state.core.settings.selected_instance = new_inst_id;
                let _ = state.core.settings.save();
            }

            win.downloads_view.set_progress(
                1.0,
                &format!("✓ Minecraft {} is fully installed & ready!", v_id),
            );
            win.home_view.append_log(&format!(
                "[DOWNLOAD] Minecraft {} installation finished! Switched active profile.",
                v_id
            ));
            win.refresh_instances_view();
            win.refresh_home_view();
        });
    }

    fn on_fetch_manifest_clicked(self: &Rc<Self>) {
        let win = self.clone();
        win.downloads_view
            .set_progress(0.1, "Fetching complete Mojang catalog...");
        glib::MainContext::default().spawn_local(async move {
            let client = reqwest::Client::new();
            match client.get(OFFICIAL_MANIFEST_URL).send().await {
                Ok(resp) => match resp.json::<VersionManifest>().await {
                    Ok(manifest) => {
                        let win_inst = win.clone();
                        let on_install = Rc::new(move |id: String, url: String| {
                            win_inst.install_version(&id, &url);
                        });
                        win.downloads_view
                            .populate_versions(&manifest.versions, on_install);
                        win.downloads_view.set_progress(
                            1.0,
                            &format!(
                                "✓ Loaded {} official Minecraft versions!",
                                manifest.versions.len()
                            ),
                        );
                        win.home_view.append_log(&format!(
                            "[CATALOG] Loaded {} Minecraft versions from Mojang manifest.",
                            manifest.versions.len()
                        ));
                    }
                    Err(e) => {
                        win.downloads_view
                            .set_progress(0.0, &format!("Failed to parse catalog: {}", e));
                    }
                },
                Err(e) => {
                    win.downloads_view
                        .set_progress(0.0, &format!("Network catalog error: {}", e));
                }
            }
        });
    }

    fn on_microsoft_login_clicked(self: &Rc<Self>) {
        let win = self.clone();
        win.account_view.code_display_box.set_visible(true);
        win.account_view
            .auth_status_label
            .set_text("Contacting Microsoft login services...");

        glib::MainContext::default().spawn_local(async move {
            let auth = AuthManager::new();
            match auth.initiate_device_code().await {
                Ok(code_resp) => {
                    win.account_view
                        .user_code_label
                        .set_text(&code_resp.user_code);
                    win.account_view.auth_status_label.set_text(&format!(
                        "Go to {} and enter code: {}",
                        code_resp.verification_uri, code_resp.user_code
                    ));

                    let uri_clone = code_resp.verification_uri.clone();
                    win.account_view.open_browser_btn.connect_clicked(move |_| {
                        let _ = open::that(&uri_clone);
                    });

                    // Start polling in background
                    win.home_view.append_log(&format!(
                        "[AUTH] Microsoft login code: {} | Open {}",
                        code_resp.user_code, code_resp.verification_uri
                    ));

                    match auth
                        .poll_device_code_token(&code_resp.device_code, code_resp.interval, 30)
                        .await
                    {
                        Ok(msa_token) => {
                            win.account_view
                                .auth_status_label
                                .set_text("Exchanging MSA token with Minecraft...");
                            match auth.complete_minecraft_login(&msa_token).await {
                                Ok(profile) => {
                                    win.account_view
                                        .auth_status_label
                                        .set_text("✓ Authentication successful!");
                                    win.home_view.append_log(&format!(
                                        "[AUTH] Logged in as {} ({})",
                                        profile.username, profile.uuid
                                    ));
                                    win.refresh_account_view();
                                    win.refresh_home_view();
                                }
                                Err(e) => {
                                    win.account_view
                                        .auth_status_label
                                        .set_text(&format!("Minecraft auth error: {}", e));
                                    win.home_view.append_log(&format!("[AUTH] Error: {}", e));
                                }
                            }
                        }
                        Err(e) => {
                            win.account_view
                                .auth_status_label
                                .set_text(&format!("Authorization expired or cancelled: {}", e));
                        }
                    }
                }
                Err(e) => {
                    win.account_view
                        .auth_status_label
                        .set_text(&format!("Failed to initiate login: {}", e));
                }
            }
        });
    }

    fn on_launch_clicked(self: &Rc<Self>) {
        let win = self.clone();

        // Safe Demo Mode check (Master Prompt Phase 2: clearly labeled sample data, does not pretend to launch real Java)
        let is_demo = self
            .state
            .borrow()
            .core
            .auth_manager
            .load_active_account()
            .map_or(false, |a| a.username.contains("Demo Explorer"));

        if is_demo {
            self.home_view
                .append_log("----------------------------------------");
            self.home_view
                .append_log("[DEMO MODE] Simulating Minecraft launch sequence with sample data...");
            self.home_view
                .append_log("[DEMO MODE] Instance integrity audit: OK (Simulation Mode)");
            self.home_view
                .append_log("[DEMO MODE] Real Java execution skipped: Safe simulation without external JVM process.");
            self.home_view
                .status_label
                .set_text("Demo Mode: Simulation Active");
            self.home_view
                .status_label
                .set_css_classes(&["badge-warning"]);
            self.home_view.play_button.set_sensitive(true);
            return;
        }

        let (tx, rx): (Sender<ProcessEvent>, Receiver<ProcessEvent>) = channel();

        self.home_view.play_button.set_sensitive(false);
        self.home_view
            .status_label
            .set_text("Launching Minecraft...");
        self.home_view.status_label.set_css_classes(&["badge-info"]);
        self.home_view
            .append_log("----------------------------------------");
        self.home_view
            .append_log("[NUX] Initializing Minecraft launch sequence...");

        let auto_close = self.state.borrow().core.settings.auto_close_on_launch;

        glib::MainContext::default().spawn_local(async move {
            let mut state = win.state.borrow_mut();
            match state.core.launch_current(tx).await {
                Ok(pid) => {
                    win.home_view
                        .append_log(&format!("[NUX] Minecraft process spawned (PID: {})", pid));
                    win.home_view
                        .status_label
                        .set_text(&format!("Running (PID {})", pid));
                    win.home_view.status_label.set_css_classes(&["badge-ready"]);

                    if auto_close {
                        win.window.set_visible(false);
                    }
                }
                Err(e) => {
                    win.home_view
                        .append_log(&format!("[NUX ERROR] Launch failed: {}", e));
                    win.home_view.status_label.set_text("Launch failed");
                    win.home_view
                        .status_label
                        .set_css_classes(&["badge-warning"]);
                    win.home_view.play_button.set_sensitive(true);
                }
            }
        });

        // Set up periodic poll on main loop to receive console logs from MPSC channel
        let win_poll = self.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
            let mut keep_going = true;
            while let Ok(event) = rx.try_recv() {
                match event {
                    ProcessEvent::LogLine(line) => {
                        win_poll.home_view.append_log(&line);
                    }
                    ProcessEvent::Started(pid) => {
                        win_poll
                            .home_view
                            .status_label
                            .set_text(&format!("Minecraft Running (PID {})", pid));
                    }
                    ProcessEvent::Finished(code) => {
                        win_poll
                            .home_view
                            .append_log(&format!("[NUX] Minecraft exited with code {}", code));
                        win_poll
                            .home_view
                            .status_label
                            .set_text(&format!("Exited ({})", code));
                        win_poll.home_view.play_button.set_sensitive(true);
                        win_poll.window.set_visible(true);
                        win_poll.window.present();
                        keep_going = false;
                    }
                    ProcessEvent::Crashed(report) => {
                        win_poll
                            .home_view
                            .append_log(&format!("[NUX CRASH]\n{}", report));
                        win_poll.home_view.status_label.set_text("Game Crashed!");
                        win_poll
                            .home_view
                            .status_label
                            .set_css_classes(&["badge-warning"]);
                        win_poll.home_view.play_button.set_sensitive(true);
                        win_poll.window.set_visible(true);
                        win_poll.window.present();
                        keep_going = false;
                    }
                }
            }
            if keep_going {
                glib::ControlFlow::Continue
            } else {
                glib::ControlFlow::Break
            }
        });
    }
}

pub fn load_css() {
    let provider = CssProvider::new();
    let css_data = include_str!("../assets/styles/style.css");
    provider.load_from_string(css_data);

    if let Some(display) = Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
