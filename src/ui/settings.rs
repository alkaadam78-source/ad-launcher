use crate::config::settings::LauncherSettings;
use crate::ui::{create_page_header, create_section_divider};
use crate::utils::system::SystemSpecs;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, CheckButton, Entry, Label, Orientation, Scale};

pub struct SettingsView {
    pub container: GtkBox,
    pub balanced_btn: Button,
    pub low_res_btn: Button,
    pub hyper_btn: Button,
    pub ram_scale: Scale,
    pub ram_display_label: Label,
    pub java_entry: Entry,
    pub jvm_args_entry: Entry,
    pub width_entry: Entry,
    pub height_entry: Entry,
    pub auto_close_check: CheckButton,
    pub enable_gc_check: CheckButton,
    pub lang_en_btn: Button,
    pub lang_id_btn: Button,
    pub export_diag_btn: Button,
    pub reset_config_btn: Button,
    pub save_btn: Button,
    pub status_label: Label,
}

impl SettingsView {
    pub fn new(specs: &SystemSpecs) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 18);
        container.set_css_classes(&["content-area"]);

        // Page Header (Rule 1: Heading with subtle ornamental separator)
        container.append(&create_page_header(
            "ROYAL PREFERENCES & ENGINE CONFIGURATION",
            "Fine-tune resource allocation, JVM parameters, and Debian graphics behavior",
        ));

        // Hardware Presets Card
        let preset_card = GtkBox::new(Orientation::Vertical, 12);
        preset_card.set_css_classes(&["card"]);

        let p_title = Label::new(Some("HARDWARE PERFORMANCE PRESETS"));
        p_title.set_css_classes(&["card-title"]);
        p_title.set_halign(gtk4::Align::Start);
        preset_card.append(&p_title);

        let p_desc = Label::new(Some(
            "Curated presets tuned for Intel Core i3-7100U, Intel HD Graphics 620, and 8 GB RAM systems."
        ));
        p_desc.set_css_classes(&["page-subtitle"]);
        p_desc.set_wrap(true);
        p_desc.set_halign(gtk4::Align::Start);
        preset_card.append(&p_desc);

        let btn_row = GtkBox::new(Orientation::Horizontal, 12);
        let balanced_btn = Button::with_label("⚜ BALANCED (3 GB, 6 Chunks)");
        balanced_btn.set_css_classes(&["btn-cyan"]);
        balanced_btn.set_hexpand(true);

        let low_res_btn = Button::with_label("⚡ LOW (2 GB, 4 Chunks)");
        low_res_btn.set_css_classes(&["btn-amber"]);
        low_res_btn.set_hexpand(true);

        let hyper_btn = Button::with_label("⚔ HYPER BOOST (ADAM TURBO - 240 FPS)");
        hyper_btn.set_css_classes(&["btn-purple"]);
        hyper_btn.set_hexpand(true);

        btn_row.append(&balanced_btn);
        btn_row.append(&low_res_btn);
        btn_row.append(&hyper_btn);
        preset_card.append(&btn_row);

        container.append(&preset_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // Memory & Resource Limits Card
        let ram_card = GtkBox::new(Orientation::Vertical, 12);
        ram_card.set_css_classes(&["card"]);

        let ram_header = GtkBox::new(Orientation::Horizontal, 12);
        let ram_title = Label::new(Some("MAXIMUM RAM ALLOCATION (MB)"));
        ram_title.set_css_classes(&["card-title"]);
        ram_title.set_hexpand(true);
        ram_title.set_halign(gtk4::Align::Start);
        ram_header.append(&ram_title);

        let ram_display_label = Label::new(Some("3072 MB"));
        ram_display_label.set_css_classes(&["badge-turbo"]);
        ram_header.append(&ram_display_label);
        ram_card.append(&ram_header);

        let ram_limit_desc = format!(
            "Total System RAM: {} MB. Recommended Safe Max: {} MB. Hard Limit: {} MB.",
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

        let enable_gc_check =
            CheckButton::with_label("Enable G1GC Low-Pause Latency Garbage Collector flags");
        enable_gc_check.set_active(true);
        ram_card.append(&enable_gc_check);

        container.append(&ram_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // Java Runtime & Arguments Card
        let java_card = GtkBox::new(Orientation::Vertical, 12);
        java_card.set_css_classes(&["card"]);

        let j_title = Label::new(Some("JAVA RUNTIME & JVM ARGUMENTS"));
        j_title.set_css_classes(&["card-title"]);
        j_title.set_halign(gtk4::Align::Start);
        java_card.append(&j_title);

        let j_desc = Label::new(Some(
            "Optional custom Java executable path. Leave blank to auto-detect system Java.",
        ));
        j_desc.set_css_classes(&["page-subtitle"]);
        j_desc.set_halign(gtk4::Align::Start);
        java_card.append(&j_desc);

        let java_entry = Entry::new();
        java_entry.set_placeholder_text(Some(
            "/usr/lib/jvm/java-17-openjdk-amd64/bin/java (blank = auto)",
        ));
        java_card.append(&java_entry);

        let a_desc = Label::new(Some("Custom JVM Arguments:"));
        a_desc.set_css_classes(&["page-subtitle"]);
        a_desc.set_margin_top(6);
        a_desc.set_halign(gtk4::Align::Start);
        java_card.append(&a_desc);

        let jvm_args_entry = Entry::new();
        jvm_args_entry.set_placeholder_text(Some(
            "-XX:+UseG1GC -XX:+UnlockExperimentalVMOptions (blank = defaults)",
        ));
        java_card.append(&jvm_args_entry);

        container.append(&java_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // Display Resolution & Lifecycle Card
        let win_card = GtkBox::new(Orientation::Vertical, 12);
        win_card.set_css_classes(&["card"]);

        let w_title = Label::new(Some("DISPLAY RESOLUTION & APPLICATION LIFECYCLE"));
        w_title.set_css_classes(&["card-title"]);
        w_title.set_halign(gtk4::Align::Start);
        win_card.append(&w_title);

        let res_row = GtkBox::new(Orientation::Horizontal, 12);

        let w_lbl = Label::new(Some("Width:"));
        w_lbl.set_css_classes(&["page-subtitle"]);
        res_row.append(&w_lbl);

        let width_entry = Entry::new();
        width_entry.set_text("1366");
        width_entry.set_width_chars(8);
        res_row.append(&width_entry);

        let h_lbl = Label::new(Some("Height:"));
        h_lbl.set_css_classes(&["page-subtitle"]);
        h_lbl.set_margin_start(8);
        res_row.append(&h_lbl);

        let height_entry = Entry::new();
        height_entry.set_text("768");
        height_entry.set_width_chars(8);
        res_row.append(&height_entry);

        win_card.append(&res_row);

        let auto_close_check =
            CheckButton::with_label("Minimize/hide launcher when Minecraft launches to save RAM");
        auto_close_check.set_active(false);
        auto_close_check.set_margin_top(6);
        win_card.append(&auto_close_check);

        container.append(&win_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // Language & Localization Card (Rule 9)
        let lang_card = GtkBox::new(Orientation::Vertical, 12);
        lang_card.set_css_classes(&["card"]);

        let l_title = Label::new(Some("LANGUAGE & LOCALIZATION (I18N)"));
        l_title.set_css_classes(&["card-title"]);
        l_title.set_halign(gtk4::Align::Start);
        lang_card.append(&l_title);

        let lang_row = GtkBox::new(Orientation::Horizontal, 10);
        let lang_en_btn = Button::with_label("🇺🇸 English (US)");
        lang_en_btn.set_css_classes(&["filter-btn", "active"]);

        let lang_id_btn = Button::with_label("🇮🇩 Bahasa Indonesia");
        lang_id_btn.set_css_classes(&["filter-btn"]);

        lang_row.append(&lang_en_btn);
        lang_row.append(&lang_id_btn);
        lang_card.append(&lang_row);

        container.append(&lang_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // Diagnostics & Troubleshooting Card
        let diag_card = GtkBox::new(Orientation::Vertical, 12);
        diag_card.set_css_classes(&["card"]);

        let d_title = Label::new(Some("TROUBLESHOOTING & SANITIZED DIAGNOSTICS"));
        d_title.set_css_classes(&["card-title"]);
        d_title.set_halign(gtk4::Align::Start);
        diag_card.append(&d_title);

        let diag_row = GtkBox::new(Orientation::Horizontal, 10);
        let export_diag_btn = Button::with_label("📋 Export Sanitized Diagnostic Report");
        export_diag_btn.set_css_classes(&["btn-cyan"]);

        let reset_config_btn = Button::with_label("↻ Reset to Balanced Defaults");
        reset_config_btn.set_css_classes(&["btn-danger"]);

        diag_row.append(&export_diag_btn);
        diag_row.append(&reset_config_btn);
        diag_card.append(&diag_row);

        container.append(&diag_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // Save Action Row
        let save_row = GtkBox::new(Orientation::Horizontal, 12);
        let save_btn = Button::with_label("💾  Save Preferences");
        save_btn.set_css_classes(&["btn-play"]);
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
            hyper_btn,
            ram_scale,
            ram_display_label,
            java_entry,
            jvm_args_entry,
            width_entry,
            height_entry,
            auto_close_check,
            enable_gc_check,
            lang_en_btn,
            lang_id_btn,
            export_diag_btn,
            reset_config_btn,
            save_btn,
            status_label,
        }
    }

    pub fn load_settings(&self, settings: &LauncherSettings) {
        self.ram_scale.set_value(settings.ram_mb as f64);
        self.ram_display_label
            .set_text(&format!("{} MB", settings.ram_mb));
        if let Some(ref p) = settings.java_path {
            self.java_entry.set_text(&p.to_string_lossy());
        }
        self.jvm_args_entry.set_text(&settings.custom_jvm_args);
        self.width_entry
            .set_text(&settings.window_width.to_string());
        self.height_entry
            .set_text(&settings.window_height.to_string());
        self.auto_close_check
            .set_active(settings.auto_close_on_launch);
        self.enable_gc_check
            .set_active(settings.enable_performance_gc);
    }
}
