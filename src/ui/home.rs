use crate::core::launcher::LauncherStatus;
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, Label, Orientation, ProgressBar, ScrolledWindow, TextView,
};

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

        // Page Header
        let header_row = GtkBox::new(Orientation::Horizontal, 12);
        
        let title_col = GtkBox::new(Orientation::Vertical, 2);
        let title = Label::new(Some("⚡ AD LAUNCHER"));
        title.set_css_classes(&["page-title"]);
        title.set_halign(gtk4::Align::Start);

        let subtitle = Label::new(Some("Extreme Potato & Cartoon Edition — Ultra Boosted for Intel HD 620"));
        subtitle.set_css_classes(&["page-subtitle"]);
        subtitle.set_halign(gtk4::Align::Start);

        title_col.append(&title);
        title_col.append(&subtitle);
        header_row.append(&title_col);

        let credit_badge = Label::new(Some("⚡ CREDIT BY ADAMDEVNOOB ⚡"));
        credit_badge.set_css_classes(&["adam-credit-badge"]);
        credit_badge.set_halign(gtk4::Align::End);
        credit_badge.set_hexpand(true);
        credit_badge.set_valign(gtk4::Align::Center);
        header_row.append(&credit_badge);

        container.append(&header_row);

        // Turbo Feature Banner Card
        let turbo_banner = GtkBox::new(Orientation::Horizontal, 12);
        turbo_banner.set_css_classes(&["banner-card"]);
        let turbo_icon = Label::new(Some("🚀"));
        turbo_icon.set_css_classes(&["page-title"]);
        let turbo_text = Label::new(Some("ADAM TURBO BOOST ENGAGED: Multi-threaded OpenGL + Potato Shaders + Zero-Stutter G1GC"));
        turbo_text.set_css_classes(&["banner-title"]);
        turbo_text.set_wrap(true);
        turbo_banner.append(&turbo_icon);
        turbo_banner.append(&turbo_text);
        container.append(&turbo_banner);

        // Instance & Launch Card
        let launch_card = GtkBox::new(Orientation::Vertical, 12);
        launch_card.set_css_classes(&["card"]);

        let card_title = Label::new(Some("ACTIVE PROFILE"));
        card_title.set_css_classes(&["card-title"]);
        card_title.set_halign(gtk4::Align::Start);
        launch_card.append(&card_title);

        let instance_label = Label::new(Some("Instance: Loading..."));
        instance_label.set_halign(gtk4::Align::Start);
        launch_card.append(&instance_label);

        let java_label = Label::new(Some("Java Runtime: Detecting..."));
        java_label.set_halign(gtk4::Align::Start);
        launch_card.append(&java_label);

        let ram_label = Label::new(Some("RAM Allocation: 3 GB (Balanced Preset)"));
        ram_label.set_halign(gtk4::Align::Start);
        launch_card.append(&ram_label);

        let perf_label = Label::new(Some("Optimization Mods: Scanning..."));
        perf_label.set_halign(gtk4::Align::Start);
        launch_card.append(&perf_label);

        // Play Button Box
        let btn_box = GtkBox::new(Orientation::Horizontal, 12);
        btn_box.set_margin_top(8);

        let play_button = Button::with_label("🔥 LAUNCH MINECRAFT 1.20.1 (TURBO BOOST) 🔥");
        play_button.set_css_classes(&["btn-play"]);
        play_button.set_hexpand(true);
        btn_box.append(&play_button);

        launch_card.append(&btn_box);

        // Progress bar
        let progress_bar = ProgressBar::new();
        progress_bar.set_fraction(0.0);
        progress_bar.set_visible(false);
        launch_card.append(&progress_bar);

        let status_label = Label::new(Some("Ready to play."));
        status_label.set_css_classes(&["badge-ready"]);
        status_label.set_halign(gtk4::Align::Start);
        launch_card.append(&status_label);

        container.append(&launch_card);

        // Activity Log Card
        let log_card = GtkBox::new(Orientation::Vertical, 8);
        log_card.set_css_classes(&["card"]);
        log_card.set_vexpand(true);

        let log_title = Label::new(Some("GAME ACTIVITY & CONSOLE LOG"));
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
            self.ram_label.set_text(&format!("RAM Allocation: {} MB", inst.ram_mb));
        }

        if let Some(ref j) = status.detected_java {
            self.java_label.set_text(&format!(
                "Java Runtime: {} ({}) [{}]",
                j.version_string,
                if j.is_64bit { "64-bit" } else { "32-bit" },
                j.path.display()
            ));
        } else {
            self.java_label.set_text("Java Runtime: None found! Please install Java 17.");
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
            self.perf_label.set_text("Performance Mods: None active. (Visit Mods tab to configure)");
        } else {
            self.perf_label.set_text(&format!(
                "Performance Mods Active: {}",
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
