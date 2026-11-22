use crate::core::launcher::LauncherStatus;
use crate::ui::components::{create_page_header, create_section_divider};
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Label, Orientation, ProgressBar, ScrolledWindow, TextView};

pub struct HomeView {
    pub container: GtkBox,
    pub status_label: Label,
    pub instance_label: Label,
    pub java_label: Label,
    pub ram_label: Label,
    pub perf_label: Label,
    pub play_button: Button,
    pub progress_bar: ProgressBar,
    pub log_view: TextView,
}

impl HomeView {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 16);
        container.set_css_classes(&["content-area"]);

        // Page Header with Subtle Ornamental Separator (Rule 1)
        let header = create_page_header(
            "AD LAUNCHER",
            "The Art of Minecraft — Dark Baroque Edition for Debian Linux",
        );
        container.append(&header);

        // Baroque Gallery Entrance Hero Banner (Rule 4: Richer Baroque framing in hero)
        let banner_card = GtkBox::new(Orientation::Vertical, 6);
        banner_card.set_css_classes(&["banner-card"]);

        let hero_badge_row = GtkBox::new(Orientation::Horizontal, 8);
        let arch_icon = Label::new(Some("⚜"));
        arch_icon.set_css_classes(&["page-title"]);
        let banner_title = Label::new(Some("GRAND GALLERY ENTRANCE • ADAM TURBO ENGINE"));
        banner_title.set_css_classes(&["banner-title"]);
        hero_badge_row.append(&arch_icon);
        hero_badge_row.append(&banner_title);
        banner_card.append(&hero_badge_row);

        let banner_desc = Label::new(Some(
            "Illuminated with multi-threaded Mesa OpenGL, Chiaroscuro shader calibration, and low-latency G1GC memory tuning for Intel HD 620."
        ));
        banner_desc.set_css_classes(&["page-subtitle"]);
        banner_desc.set_wrap(true);
        banner_desc.set_halign(gtk4::Align::Start);
        banner_card.append(&banner_desc);

        container.append(&banner_card);

        // Active Profile & Launch Card (Rule 3: Subtle corner accents, clean interior)
        let launch_card = GtkBox::new(Orientation::Vertical, 12);
        launch_card.set_css_classes(&["card"]);

        let card_header = GtkBox::new(Orientation::Horizontal, 8);
        let card_title = Label::new(Some("CURATED ACTIVE PROFILE"));
        card_title.set_css_classes(&["card-title"]);
        card_title.set_halign(gtk4::Align::Start);
        card_title.set_hexpand(true);
        card_header.append(&card_title);

        let status_label = Label::new(Some("Ready for Launch"));
        status_label.set_css_classes(&["badge-ready"]);
        status_label.set_halign(gtk4::Align::End);
        card_header.append(&status_label);
        launch_card.append(&card_header);

        let details_col = GtkBox::new(Orientation::Vertical, 4);

        let instance_label = Label::new(Some("Instance: Loading collection..."));
        instance_label.set_halign(gtk4::Align::Start);
        details_col.append(&instance_label);

        let java_label = Label::new(Some("Java Runtime: Probing environment..."));
        java_label.set_halign(gtk4::Align::Start);
        details_col.append(&java_label);

        let ram_label = Label::new(Some("RAM Allocation: 3072 MB (Balanced Baroque Preset)"));
        ram_label.set_halign(gtk4::Align::Start);
        details_col.append(&ram_label);

        let perf_label = Label::new(Some("Optimization Suite: Inspecting Sodium & Iris..."));
        perf_label.set_halign(gtk4::Align::Start);
        details_col.append(&perf_label);

        launch_card.append(&details_col);

        // Master Launch Button (Rule 10: Simple, modern, minimalist controls)
        let btn_box = GtkBox::new(Orientation::Horizontal, 12);
        btn_box.set_margin_top(6);

        let play_button = Button::with_label("⚔  LAUNCH MINECRAFT 1.20.1 (BAROQUE EDITION)  ⚔");
        play_button.set_css_classes(&["btn-play"]);
        play_button.set_hexpand(true);
        btn_box.append(&play_button);
        launch_card.append(&btn_box);

        // Progress bar
        let progress_bar = ProgressBar::new();
        progress_bar.set_fraction(0.0);
        progress_bar.set_visible(false);
        launch_card.append(&progress_bar);

        container.append(&launch_card);

        // Symmetrical Section Divider (Rule 2)
        container.append(&create_section_divider());

        // Activity Log Card (Chronicle & Console)
        let log_card = GtkBox::new(Orientation::Vertical, 8);
        log_card.set_css_classes(&["card"]);
        log_card.set_vexpand(true);

        let log_title = Label::new(Some("CHRONICLES & CONSOLE TELEMETRY"));
        log_title.set_css_classes(&["card-title"]);
        log_title.set_halign(gtk4::Align::Start);
        log_card.append(&log_title);

        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);
        scrolled.set_min_content_height(160);

        let log_view = TextView::new();
        log_view.set_editable(false);
        log_view.set_monospace(true);
        log_view.set_css_classes(&["log-view"]);
        scrolled.set_child(Some(&log_view));

        log_card.append(&scrolled);
        container.append(&log_card);

        Self {
            container,
            status_label,
            instance_label,
            java_label,
            ram_label,
            perf_label,
            play_button,
            progress_bar,
            log_view,
        }
    }

    pub fn update_status(&self, status: &LauncherStatus) {
        if let Some(ref inst) = status.selected_instance {
            self.instance_label.set_text(&format!(
                "Instance: {} (MC {} • {})",
                inst.name, inst.mc_version, inst.loader
            ));
            self.ram_label
                .set_text(&format!("RAM Allocation: {} MB", inst.ram_mb));
            self.play_button.set_label(&format!(
                "⚔  LAUNCH MINECRAFT {} (BAROQUE EDITION)  ⚔",
                inst.mc_version
            ));
        }

        if let Some(ref j) = status.detected_java {
            self.java_label.set_text(&format!(
                "Java Runtime: {} ({}) [{}]",
                j.version_string,
                if j.is_64bit { "64-bit" } else { "32-bit" },
                j.path.display()
            ));
        } else {
            self.java_label
                .set_text("Java Runtime: None found! Please configure Java 17+.");
        }

        let p = &status.performance_status;
        let mut mods_found = Vec::new();
        if p.has_sodium {
            mods_found.push("Sodium");
        }
        if p.has_iris {
            mods_found.push("Iris");
        }
        if p.has_lithium {
            mods_found.push("Lithium");
        }
        if p.has_ferritecore {
            mods_found.push("FerriteCore");
        }

        if mods_found.is_empty() {
            self.perf_label
                .set_text("Performance Suite: None active. (Visit Mods archive to configure)");
        } else {
            self.perf_label.set_text(&format!(
                "Performance Suite: Active ({})",
                mods_found.join(", ")
            ));
        }

        self.status_label.set_text(&status.status_message);
        if status.is_ready {
            self.status_label.set_css_classes(&["badge-ready"]);
            self.play_button.set_sensitive(true);
        } else {
            self.status_label.set_css_classes(&["badge-warning"]);
            self.play_button.set_sensitive(false);
        }
    }

    pub fn append_log(&self, line: &str) {
        let buffer = self.log_view.buffer();
        let mut end_iter = buffer.end_iter();
        buffer.insert(&mut end_iter, &format!("{}\n", line));
    }
}
