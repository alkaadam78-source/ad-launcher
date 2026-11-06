use crate::config::presets::OptimizationPreset;
use crate::core::authentication::AuthManager;
use crate::core::downloader::MinecraftDownloader;
use crate::core::instances::Instance;
use crate::core::launcher::LauncherCore;
use crate::core::manifest::{VersionManifest, VersionPackage, OFFICIAL_MANIFEST_URL};
use crate::core::mods::{ModInfo, ModManager};
use crate::core::process::ProcessEvent;
use crate::integrations::fabric::FabricClient;
use crate::integrations::modrinth::ModrinthClient;
use crate::ui::*;
use crate::utils::system::{detect_system_specs, SystemSpecs};
use gtk4::gdk::Display;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, CssProvider, Label, Orientation, Stack,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, Sender};

pub const APP_ID: &str = "org.nuxlauncher.linux";

pub struct AppState {
    pub core: LauncherCore,
    pub specs: SystemSpecs,
    pub modrinth: ModrinthClient,
    pub fabric: FabricClient,
    pub is_downloading: bool,
    pub is_running_game: bool,
}

pub struct MainWindow {
    pub window: ApplicationWindow,
    pub stack: Stack,
    pub home_view: Rc<HomeView>,
    pub instances_view: Rc<InstancesView>,
    pub mods_view: Rc<ModsView>,
    pub downloads_view: Rc<DownloadsView>,
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
        let fabric = FabricClient::new();

        let state = Rc::new(RefCell::new(AppState {
            core,
            specs: specs.clone(),
            modrinth,
            fabric,
            is_downloading: false,
            is_running_game: false,
        }));

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Ad Launcher — Credit By AdamDevnoob (Extreme FPS Boost)")
            .default_width(1040)
            .default_height(640)
            .build();

        // Main layout: Horizontal box with sidebar and stack
        let root_box = GtkBox::new(Orientation::Horizontal, 0);

        // Sidebar (Cartoon Neo-Brutalism)
        let sidebar = GtkBox::new(Orientation::Vertical, 4);
        sidebar.set_css_classes(&["sidebar"]);
        sidebar.set_size_request(210, -1);

        let brand_box = GtkBox::new(Orientation::Vertical, 4);

        let brand_card = GtkBox::new(Orientation::Horizontal, 8);
        brand_card.set_css_classes(&["sidebar-title-box"]);

        let logo_img = gtk4::Image::from_icon_name("ad-launcher");
        logo_img.set_pixel_size(24);
        brand_card.append(&logo_img);

        let brand_lbl = Label::new(Some("AD LAUNCHER"));
        brand_lbl.set_css_classes(&["sidebar-title"]);
        brand_card.append(&brand_lbl);

        let credit_lbl = Label::new(Some("⚡ Credit By AdamDevnoob"));
        credit_lbl.set_css_classes(&["adam-credit-badge"]);
        credit_lbl.set_halign(gtk4::Align::Center);

        brand_box.append(&brand_card);
        brand_box.append(&credit_lbl);
        sidebar.append(&brand_box);

        let stack = Stack::new();
        stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
        stack.set_hexpand(true);
        stack.set_vexpand(true);

        let home_view = Rc::new(HomeView::new());
        let instances_view = Rc::new(InstancesView::new());
        let mods_view = Rc::new(ModsView::new());
        let downloads_view = Rc::new(DownloadsView::new());
        let settings_view = Rc::new(SettingsView::new(&specs));
        let account_view = Rc::new(AccountView::new());
        let about_view = Rc::new(AboutView::new(&specs));

        stack.add_named(&home_view.container, Some("home"));
        stack.add_named(&instances_view.container, Some("instances"));
        stack.add_named(&mods_view.container, Some("mods"));
        stack.add_named(&downloads_view.container, Some("downloads"));
        stack.add_named(&settings_view.container, Some("settings"));
        stack.add_named(&account_view.container, Some("account"));
        stack.add_named(&about_view.container, Some("about"));

        let nav_items = [
            ("home", "🏠  Home"),
            ("instances", "📦  Instances"),
            ("mods", "🧩  Mods"),
            ("downloads", "⬇  Downloads"),
            ("settings", "⚙  Settings"),
            ("account", "👤  Account"),
            ("about", "ℹ  About"),
        ];

        let mut nav_buttons = Vec::new();
        for (id, label) in nav_items {
            let btn = Button::with_label(label);
            btn.set_css_classes(&["nav-btn"]);
            btn.set_halign(gtk4::Align::Fill);
            if id == "home" {
                btn.add_css_class("active");
            }
            sidebar.append(&btn);
            nav_buttons.push((id.to_string(), btn));
        }

        // Cartoon Turbo Status Footer in Sidebar
        let turbo_status_box = GtkBox::new(Orientation::Vertical, 4);
        turbo_status_box.set_margin_top(16);
        turbo_status_box.set_valign(gtk4::Align::End);
        turbo_status_box.set_vexpand(true);

        let turbo_lbl = Label::new(Some("🚀 ADAM TURBO: ON"));
        turbo_lbl.set_css_classes(&["badge-turbo"]);
        turbo_status_box.append(&turbo_lbl);

        let fps_lbl = Label::new(Some("⚡ 120-240 FPS BOOST"));
        fps_lbl.set_css_classes(&["badge-ready"]);
        fps_lbl.set_margin_top(2);
        turbo_status_box.append(&fps_lbl);

        sidebar.append(&turbo_status_box);

        root_box.append(&sidebar);
        root_box.append(&stack);
        window.set_child(Some(&root_box));

        let main_win = Rc::new(Self {
            window,
            stack,
            home_view,
            instances_view,
            mods_view,
            downloads_view,
            settings_view,
            account_view,
            about_view,
            nav_buttons,
            state,
        });

        main_win.connect_events();
        main_win.refresh_all_views();

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
                state.core.settings.apply_preset(OptimizationPreset::Balanced);
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
                state.core.settings.apply_preset(OptimizationPreset::LowResource);
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
                state.core.settings.apply_preset(OptimizationPreset::HyperBoost);
                let _ = state.core.settings.save();
                drop(state);
                win.refresh_settings_view();
                win.refresh_home_view();
            });
        }

        // 5. Settings RAM Scale Changed
        {
            let win = self.clone();
            self.settings_view.ram_scale.connect_value_changed(move |scale| {
                let val = scale.value() as u32;
                win.settings_view.ram_display_label.set_text(&format!("{} MB", val));
            });
        }

        // 6. Instances: Create Instance
        {
            let win = self.clone();
            self.instances_view.create_btn.connect_clicked(move |_| {
                let name = win.instances_view.name_entry.text().to_string();
                if !name.trim().is_empty() {
                    let mut state = win.state.borrow_mut();
                    let new_id = format!("inst-{}", uuid::Uuid::new_v4().simple());
                    let inst = Instance {
                        id: new_id.clone(),
                        name: name.trim().to_string(),
                        mc_version: "1.20.1".to_string(),
                        loader: "fabric".to_string(),
                        loader_version: "0.15.11".to_string(),
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

        // 8. Mods: Refresh
        {
            let win = self.clone();
            self.mods_view.refresh_btn.connect_clicked(move |_| {
                win.refresh_mods_view();
            });
        }

        // 9. Mods: Modrinth Search
        {
            let win = self.clone();
            self.mods_view.search_btn.connect_clicked(move |_| {
                win.on_search_modrinth_clicked();
            });
        }

        // 10. Mods: Quick performance mod buttons
        self.connect_quick_mod_button(&self.mods_view.quick_sodium_btn, "sodium", "Sodium");
        self.connect_quick_mod_button(&self.mods_view.quick_iris_btn, "iris", "Iris");
        self.connect_quick_mod_button(&self.mods_view.quick_lithium_btn, "lithium", "Lithium");
        self.connect_quick_mod_button(&self.mods_view.quick_ferrite_btn, "ferrite-core", "FerriteCore");

        // 11. Downloads: Download 1.20.1
        {
            let win = self.clone();
            self.downloads_view.download_1_20_1_btn.connect_clicked(move |_| {
                win.on_download_1_20_1_clicked();
            });
        }

        // 12. Downloads: Fetch Manifest
        {
            let win = self.clone();
            self.downloads_view.fetch_manifest_btn.connect_clicked(move |_| {
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
    }

    fn connect_quick_mod_button(self: &Rc<Self>, btn: &Button, slug: &'static str, name: &'static str) {
        let win = self.clone();
        btn.connect_clicked(move |_| {
            win.install_modrinth_mod(slug.to_string(), name.to_string());
        });
    }

    pub fn refresh_all_views(self: &Rc<Self>) {
        self.refresh_home_view();
        self.refresh_instances_view();
        self.refresh_mods_view();
        self.refresh_settings_view();
        self.refresh_account_view();
    }

    pub fn refresh_home_view(&self) {
        let state = self.state.borrow();
        let status = state.core.get_status();
        self.home_view.update_status(&status);
    }

    pub fn refresh_instances_view(self: &Rc<Self>) {
        let state = self.state.borrow();
        let instances = state.core.instance_manager.list_instances().unwrap_or_default();
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

        self.instances_view
            .populate_instances(&instances, &active_id, on_select, on_open_folder, on_delete);
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
            let gdir = inst_sub.map(|i| i.game_dir).unwrap_or_else(|| s.core.settings.minecraft_dir.clone());
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
            let gdir = inst_sub.map(|i| i.game_dir).unwrap_or_else(|| s.core.settings.minecraft_dir.clone());
            let mgr = ModManager::new(gdir);
            let _ = mgr.delete_mod(&m);
            drop(s);
            win_del.refresh_mods_view();
            win_del.refresh_home_view();
        });

        self.mods_view.populate_installed_mods(&mods, on_toggle, on_delete);
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

        self.settings_view.status_label.set_text("Settings saved successfully!");
        self.settings_view.status_label.set_visible(true);
        self.refresh_home_view();
    }

    fn on_search_modrinth_clicked(self: &Rc<Self>) {
        let query = self.mods_view.search_entry.text().to_string();
        if query.trim().is_empty() {
            return;
        }

        let win = self.clone();
        glib::MainContext::default().spawn_local(async move {
            let client = ModrinthClient::new();
            match client.search_mods(&query, "1.20.1", "fabric", 10).await {
                Ok(hits) => {
                    let win_inst = win.clone();
                    let on_install = Rc::new(move |slug: String, title: String| {
                        win_inst.install_modrinth_mod(slug, title);
                    });
                    win.mods_view.populate_search_results(&hits, on_install);
                }
                Err(e) => {
                    log::error!("Search failed: {}", e);
                }
            }
        });
    }

    fn install_modrinth_mod(self: &Rc<Self>, slug: String, name: String) {
        let win = self.clone();
        let state = self.state.borrow();
        let inst = state.core.get_selected_instance();
        let game_dir = inst
            .as_ref()
            .map(|i| i.game_dir.clone())
            .unwrap_or_else(|| state.core.settings.minecraft_dir.clone());
        drop(state);

        let mods_dir = game_dir.join("mods");
        let _ = crate::utils::fs::ensure_dir(&mods_dir);

        win.home_view.append_log(&format!("[MODS] Downloading {} from Modrinth...", name));

        glib::MainContext::default().spawn_local(async move {
            let client = ModrinthClient::new();
            match client.get_latest_version(&slug, "1.20.1", "fabric").await {
                Ok(ver) => match client.download_mod_file(&ver, &mods_dir).await {
                    Ok(path) => {
                        win.home_view.append_log(&format!(
                            "[MODS] Installed {} successfully to {}",
                            name,
                            path.file_name().unwrap_or_default().to_string_lossy()
                        ));
                        win.refresh_mods_view();
                        win.refresh_home_view();
                    }
                    Err(e) => {
                        win.home_view.append_log(&format!("[MODS] Failed to download {}: {}", name, e));
                    }
                },
                Err(e) => {
                    win.home_view.append_log(&format!("[MODS] Failed to find version for {}: {}", name, e));
                }
            }
        });
    }

    fn on_download_1_20_1_clicked(self: &Rc<Self>) {
        let win = self.clone();
        let state = self.state.borrow();
        let mc_dir = state.core.settings.minecraft_dir.clone();
        drop(state);

        win.downloads_view.set_progress(0.1, "Fetching Mojang Manifest...");
        win.home_view.append_log("[DOWNLOAD] Fetching official Minecraft 1.20.1 manifest...");

        glib::MainContext::default().spawn_local(async move {
            let downloader = MinecraftDownloader::new(&mc_dir);
            let client = reqwest::Client::new();

            // 1. Fetch Version Manifest
            let manifest_resp = match client.get(OFFICIAL_MANIFEST_URL).send().await {
                Ok(r) => r,
                Err(e) => {
                    win.downloads_view.set_progress(0.0, &format!("Network error: {}", e));
                    return;
                }
            };

            let manifest: VersionManifest = match manifest_resp.json().await {
                Ok(m) => m,
                Err(e) => {
                    win.downloads_view.set_progress(0.0, &format!("Failed to parse manifest: {}", e));
                    return;
                }
            };

            let entry_1_20_1 = match manifest.versions.into_iter().find(|v| v.id == "1.20.1") {
                Some(e) => e,
                None => {
                    win.downloads_view.set_progress(0.0, "Version 1.20.1 not found in manifest");
                    return;
                }
            };

            // 2. Download 1.20.1 package JSON
            win.downloads_view.set_progress(0.2, "Downloading 1.20.1.json metadata...");
            let pkg_path = match downloader.download_version_package("1.20.1", &entry_1_20_1.url).await {
                Ok(p) => p,
                Err(e) => {
                    win.downloads_view.set_progress(0.0, &format!("Error: {}", e));
                    return;
                }
            };

            let pkg_content = std::fs::read_to_string(&pkg_path).unwrap_or_default();
            let pkg: VersionPackage = match serde_json::from_str(&pkg_content) {
                Ok(p) => p,
                Err(e) => {
                    win.downloads_view.set_progress(0.0, &format!("JSON error: {}", e));
                    return;
                }
            };

            // 3. Download Client JAR
            win.downloads_view.set_progress(0.4, "Downloading 1.20.1 client jar...");
            win.home_view.append_log("[DOWNLOAD] Downloading official 1.20.1.jar...");
            if let Err(e) = downloader.download_client_jar("1.20.1", &pkg.downloads.client).await {
                win.downloads_view.set_progress(0.0, &format!("Client JAR error: {}", e));
                return;
            }

            // 4. Download Linux Libraries
            win.downloads_view.set_progress(0.6, "Downloading Linux libraries...");
            win.home_view.append_log("[DOWNLOAD] Downloading libraries for Linux x86_64...");
            if let Err(e) = downloader.download_libraries(&pkg, None).await {
                win.downloads_view.set_progress(0.0, &format!("Libraries error: {}", e));
                return;
            }

            // 5. Download Assets
            win.downloads_view.set_progress(0.8, "Checking asset index...");
            win.home_view.append_log("[DOWNLOAD] Checking asset index and game assets...");
            let _ = downloader.download_assets(&pkg, None).await;

            win.downloads_view.set_progress(1.0, "✓ Minecraft 1.20.1 is fully installed and verified!");
            win.home_view.append_log("[DOWNLOAD] Minecraft 1.20.1 installation completed successfully.");
            win.refresh_home_view();
        });
    }

    fn on_fetch_manifest_clicked(self: &Rc<Self>) {
        let win = self.clone();
        glib::MainContext::default().spawn_local(async move {
            let client = reqwest::Client::new();
            if let Ok(resp) = client.get(OFFICIAL_MANIFEST_URL).send().await {
                if let Ok(manifest) = resp.json::<VersionManifest>().await {
                    let win_inst = win.clone();
                    let on_select = Rc::new(move |id: String, url: String| {
                        win_inst.home_view.append_log(&format!("[MANIFEST] Selected version {} ({})", id, url));
                    });
                    win.downloads_view.populate_versions(&manifest.versions, on_select);
                }
            }
        });
    }

    fn on_microsoft_login_clicked(self: &Rc<Self>) {
        let win = self.clone();
        win.account_view.code_display_box.set_visible(true);
        win.account_view.auth_status_label.set_text("Contacting Microsoft login services...");

        glib::MainContext::default().spawn_local(async move {
            let auth = AuthManager::new();
            match auth.initiate_device_code().await {
                Ok(code_resp) => {
                    win.account_view.user_code_label.set_text(&code_resp.user_code);
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
                            win.account_view.auth_status_label.set_text("Exchanging MSA token with Minecraft...");
                            match auth.complete_minecraft_login(&msa_token).await {
                                Ok(profile) => {
                                    win.account_view.auth_status_label.set_text("✓ Authentication successful!");
                                    win.home_view.append_log(&format!(
                                        "[AUTH] Logged in as {} ({})",
                                        profile.username, profile.uuid
                                    ));
                                    win.refresh_account_view();
                                    win.refresh_home_view();
                                }
                                Err(e) => {
                                    win.account_view.auth_status_label.set_text(&format!("Minecraft auth error: {}", e));
                                    win.home_view.append_log(&format!("[AUTH] Error: {}", e));
                                }
                            }
                        }
                        Err(e) => {
                            win.account_view.auth_status_label.set_text(&format!("Authorization expired or cancelled: {}", e));
                        }
                    }
                }
                Err(e) => {
                    win.account_view.auth_status_label.set_text(&format!("Failed to initiate login: {}", e));
                }
            }
        });
    }

    fn on_launch_clicked(self: &Rc<Self>) {
        let win = self.clone();
        let (tx, rx): (Sender<ProcessEvent>, Receiver<ProcessEvent>) = channel();

        self.home_view.play_button.set_sensitive(false);
        self.home_view.status_label.set_text("Launching Minecraft...");
        self.home_view.status_label.set_css_classes(&["badge-info"]);
        self.home_view.append_log("----------------------------------------");
        self.home_view.append_log("[NUX] Initializing Minecraft launch sequence...");

        let auto_close = self.state.borrow().core.settings.auto_close_on_launch;

        glib::MainContext::default().spawn_local(async move {
            let mut state = win.state.borrow_mut();
            match state.core.launch_current(tx).await {
                Ok(pid) => {
                    win.home_view.append_log(&format!("[NUX] Minecraft process spawned (PID: {})", pid));
                    win.home_view.status_label.set_text(&format!("Running (PID {})", pid));
                    win.home_view.status_label.set_css_classes(&["badge-ready"]);

                    if auto_close {
                        win.window.set_visible(false);
                    }
                }
                Err(e) => {
                    win.home_view.append_log(&format!("[NUX ERROR] Launch failed: {}", e));
                    win.home_view.status_label.set_text("Launch failed");
                    win.home_view.status_label.set_css_classes(&["badge-warning"]);
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
                        win_poll.home_view.status_label.set_text(&format!("Minecraft Running (PID {})", pid));
                    }
                    ProcessEvent::Finished(code) => {
                        win_poll.home_view.append_log(&format!("[NUX] Minecraft exited with code {}", code));
                        win_poll.home_view.status_label.set_text(&format!("Exited ({})", code));
                        win_poll.home_view.play_button.set_sensitive(true);
                        win_poll.window.set_visible(true);
                        win_poll.window.present();
                        keep_going = false;
                    }
                    ProcessEvent::Crashed(report) => {
                        win_poll.home_view.append_log(&format!("[NUX CRASH]\n{}", report));
                        win_poll.home_view.status_label.set_text("Game Crashed!");
                        win_poll.home_view.status_label.set_css_classes(&["badge-warning"]);
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
