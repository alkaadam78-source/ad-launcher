use crate::config::settings::LauncherSettings;
use crate::utils::system::SystemSpecs;
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, CheckButton, Entry, Label, Orientation, Scale,
};

pub struct SettingsView {
    pub container: GtkBox,
    pub balanced_btn: Button,
    pub low_res_btn: Button,
    pub ram_scale: Scale,
    pub ram_display_label: Label,
    pub java_entry: Entry,
    pub jvm_args_entry: Entry,
    pub width_entry: Entry,
    pub height_entry: Entry,
    pub auto_close_check: CheckButton,
    pub enable_gc_check: CheckButton,
    pub save_btn: Button,
    pub status_label: Label,
}

impl SettingsView {
    pub fn new(specs: &SystemSpecs) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 16);
        container.set_css_classes(&["content-area"]);

        let title = Label::new(Some("SETTINGS & OPTIMIZATION PRESETS"));
        title.set_css_classes(&["page-title"]);
        title.set_halign(gtk4::Align::Start);

        let subtitle = Label::new(Some("Fine-tune resource usage, JVM memory, and Debian performance"));
        subtitle.set_css_classes(&["page-subtitle"]);
        subtitle.set_halign(gtk4::Align::Start);

        container.append(&title);
        container.append(&subtitle);

        // Hardware Presets Card
        let preset_card = GtkBox::new(Orientation::Vertical, 10);
        preset_card.set_css_classes(&["card"]);

        let p_title = Label::new(Some("HARDWARE PERFORMANCE PRESETS"));
        p_title.set_css_classes(&["card-title"]);
        p_title.set_halign(gtk4::Align::Start);
        preset_card.append(&p_title);

        let p_desc = Label::new(Some(
            "Quick presets pre-tuned for Intel Core i3-7100U, Intel HD Graphics 620, and 8 GB RAM."
        ));
        p_desc.set_css_classes(&["page-subtitle"]);
        p_desc.set_wrap(true);
        p_desc.set_halign(gtk4::Align::Start);
        preset_card.append(&p_desc);

        let btn_row = GtkBox::new(Orientation::Horizontal, 12);
        let balanced_btn = Button::with_label("★ BALANCED (3 GB RAM, 6 Chunks)");
        balanced_btn.set_css_classes(&["btn-primary"]);
        balanced_btn.set_hexpand(true);

        let low_res_btn = Button::with_label("⚡ LOW RESOURCE (2 GB RAM, 4 Chunks)");
        low_res_btn.set_css_classes(&["btn-secondary"]);
        low_res_btn.set_hexpand(true);

        btn_row.append(&balanced_btn);
        btn_row.append(&low_res_btn);
        preset_card.append(&btn_row);

        container.append(&preset_card);

        // Memory & Resource Limits Card
        let ram_card = GtkBox::new(Orientation::Vertical, 10);
        ram_card.set_css_classes(&["card"]);

        let ram_header = GtkBox::new(Orientation::Horizontal, 12);
        let ram_title = Label::new(Some("MAXIMUM RAM ALLOCATION (MB)"));
        ram_title.set_css_classes(&["card-title"]);
        ram_title.set_hexpand(true);
        ram_title.set_halign(gtk4::Align::Start);
        ram_header.append(&ram_title);

        let ram_display_label = Label::new(Some("3072 MB"));
        ram_display_label.set_css_classes(&["badge-info"]);
        ram_header.append(&ram_display_label);
        ram_card.append(&ram_header);

        let ram_limit_desc = format!(
            "Total System RAM: {} MB. Recommended Max: {} MB. Hard Safe Limit: {} MB.",
            specs.total_memory_mb, specs.recommended_ram_mb, specs.max_safe_ram_mb
        );
        let ram_limit_lbl = Label::new(Some(&ram_limit_desc));
        ram_limit_lbl.set_css_classes(&["page-subtitle"]);
        ram_limit_lbl.set_halign(gtk4::Align::Start);
        ram_card.append(&ram_limit_lbl);

        let max_val = (specs.max_safe_ram_mb as f64).max(4096.0);
        let ram_scale = Scale::with_range(Orientation::Horizontal, 1024.0, max_val, 256.0);
        ram_scale.set_value(3072.0);
        ram_scale.set_draw_value(false);
        ram_card.append(&ram_scale);

        let enable_gc_check = CheckButton::with_label("Enable G1GC Low-Pause Latency Garbage Collector flags");
        enable_gc_check.set_active(true);
        ram_card.append(&enable_gc_check);

        container.append(&ram_card);

        // Java & Arguments Card
        let java_card = GtkBox::new(Orientation::Vertical, 10);
        java_card.set_css_classes(&["card"]);

        let j_title = Label::new(Some("JAVA RUNTIME & JVM ARGUMENTS"));
        j_title.set_css_classes(&["card-title"]);
        j_title.set_halign(gtk4::Align::Start);
        java_card.append(&j_title);

        let java_entry = Entry::new();
        java_entry.set_placeholder_text(Some("Path to java binary (e.g. /home/alz/.minecraft/runtime/java-17/bin/java)"));
        java_card.append(&java_entry);

        let jvm_args_entry = Entry::new();
        jvm_args_entry.set_placeholder_text(Some("Additional custom JVM flags (optional)"));
        java_card.append(&jvm_args_entry);

        container.append(&java_card);

        // Display & Resolution Card
        let disp_card = GtkBox::new(Orientation::Vertical, 10);
        disp_card.set_css_classes(&["card"]);

        let d_title = Label::new(Some("GAME RESOLUTION & LAUNCH BEHAVIOR"));
        d_title.set_css_classes(&["card-title"]);
        d_title.set_halign(gtk4::Align::Start);
        disp_card.append(&d_title);

        let res_row = GtkBox::new(Orientation::Horizontal, 12);
        let width_entry = Entry::new();
        width_entry.set_placeholder_text(Some("Width (e.g. 1024)"));
        width_entry.set_text("1024");
        res_row.append(&width_entry);

        let height_entry = Entry::new();
        height_entry.set_placeholder_text(Some("Height (e.g. 640)"));
        height_entry.set_text("640");
        res_row.append(&height_entry);
        disp_card.append(&res_row);

        let auto_close_check = CheckButton::with_label("Close launcher window when Minecraft starts");
        auto_close_check.set_active(false);
        disp_card.append(&auto_close_check);

        container.append(&disp_card);

        // Save Button Row
        let save_row = GtkBox::new(Orientation::Horizontal, 12);
        let save_btn = Button::with_label("✓ Save Launcher Configuration");
        save_btn.set_css_classes(&["btn-primary"]);
        save_btn.set_hexpand(true);
        save_row.append(&save_btn);

        let status_label = Label::new(Some(""));
        status_label.set_css_classes(&["badge-ready"]);
        status_label.set_visible(false);
        save_row.append(&status_label);

        container.append(&save_row);

        Self {
            container,
            balanced_btn,
            low_res_btn,
            ram_scale,
            ram_display_label,
            java_entry,
            jvm_args_entry,
            width_entry,
            height_entry,
            auto_close_check,
            enable_gc_check,
            save_btn,
            status_label,
        }
    }

    pub fn load_settings(&self, settings: &LauncherSettings) {
        self.ram_scale.set_value(settings.ram_mb as f64);
        self.ram_display_label.set_text(&format!("{} MB", settings.ram_mb));
        if let Some(ref p) = settings.java_path {
            self.java_entry.set_text(&p.to_string_lossy());
        }
        self.jvm_args_entry.set_text(&settings.custom_jvm_args);
        self.width_entry.set_text(&settings.window_width.to_string());
        self.height_entry.set_text(&settings.window_height.to_string());
        self.auto_close_check.set_active(settings.auto_close_on_launch);
        self.enable_gc_check.set_active(settings.enable_performance_gc);
    }
}
