use crate::core::storage::StorageBreakdown;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Label, Orientation, ProgressBar};

pub struct StorageIntelligenceView {
    pub container: GtkBox,
    pub clean_logs_btn: Button,
    pub rescan_btn: Button,
    pub status_label: Label,
    pub total_label: Label,
    pub assets_label: Label,
    pub libs_label: Label,
    pub mods_label: Label,
    pub shaders_label: Label,
    pub logs_label: Label,
    pub storage_bar: ProgressBar,
}

impl StorageIntelligenceView {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 16);
        container.set_css_classes(&["content-area"]);

        // Page Header
        let header_row = GtkBox::new(Orientation::Horizontal, 12);
        let title_col = GtkBox::new(Orientation::Vertical, 2);

        let title = Label::new(Some("💾 MINECRAFT STORAGE INTELLIGENCE"));
        title.set_css_classes(&["page-title"]);
        title.set_halign(gtk4::Align::Start);

        let subtitle = Label::new(Some("Disk Space Diagnostics, Asset Verification, and Cache Cleaner"));
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

        // Quick Actions Card
        let action_card = GtkBox::new(Orientation::Vertical, 10);
        action_card.set_css_classes(&["card"]);

        let a_title = Label::new(Some("STORAGE CLEANUP & MAINTENANCE"));
        a_title.set_css_classes(&["card-title"]);
        a_title.set_halign(gtk4::Align::Start);
        action_card.append(&a_title);

        let btn_row = GtkBox::new(Orientation::Horizontal, 12);
        let rescan_btn = Button::with_label("🔍 Rescan Storage Usage");
        rescan_btn.set_css_classes(&["btn-secondary"]);

        let clean_logs_btn = Button::with_label("🧹 Clean Old Logs & Crash Reports");
        clean_logs_btn.set_css_classes(&["btn-primary"]);

        btn_row.append(&rescan_btn);
        btn_row.append(&clean_logs_btn);
        action_card.append(&btn_row);

        let status_label = Label::new(Some("Storage status: Ready."));
        status_label.set_css_classes(&["badge-ready"]);
        status_label.set_halign(gtk4::Align::Start);
        action_card.append(&status_label);

        container.append(&action_card);

        // Storage Breakdown Cards
        let breakdown_card = GtkBox::new(Orientation::Vertical, 12);
        breakdown_card.set_css_classes(&["card"]);

        let b_title = Label::new(Some("MINECRAFT DISK USAGE BREAKDOWN (~/.minecraft)"));
        b_title.set_css_classes(&["card-title"]);
        b_title.set_halign(gtk4::Align::Start);
        breakdown_card.append(&b_title);

        let storage_bar = ProgressBar::new();
        storage_bar.set_fraction(0.85);
        breakdown_card.append(&storage_bar);

        let total_label = Label::new(Some("Total Footprint: Calculating..."));
        total_label.set_css_classes(&["badge-turbo"]);
        total_label.set_halign(gtk4::Align::Start);
        breakdown_card.append(&total_label);

        let stats_col = GtkBox::new(Orientation::Vertical, 6);

        let assets_label = Label::new(Some("• Asset Objects: Calculating..."));
        assets_label.set_halign(gtk4::Align::Start);
        stats_col.append(&assets_label);

        let libs_label = Label::new(Some("• Java & Linux Native Libraries: Calculating..."));
        libs_label.set_halign(gtk4::Align::Start);
        stats_col.append(&libs_label);

        let mods_label = Label::new(Some("• Fabric Mods (.jar): Calculating..."));
        mods_label.set_halign(gtk4::Align::Start);
        stats_col.append(&mods_label);

        let shaders_label = Label::new(Some("• Shaderpacks (.zip): Calculating..."));
        shaders_label.set_halign(gtk4::Align::Start);
        stats_col.append(&shaders_label);

        let logs_label = Label::new(Some("• Logs & Crash Dumps: Calculating..."));
        logs_label.set_halign(gtk4::Align::Start);
        stats_col.append(&logs_label);

        breakdown_card.append(&stats_col);
        container.append(&breakdown_card);

        Self {
            container,
            clean_logs_btn,
            rescan_btn,
            status_label,
            total_label,
            assets_label,
            libs_label,
            mods_label,
            shaders_label,
            logs_label,
            storage_bar,
        }
    }

    pub fn update_stats(&self, stats: &StorageBreakdown) {
        self.total_label.set_text(&format!("Total Minecraft Directory: {:.1} MB", stats.total_mb));
        self.assets_label.set_text(&format!("• Assets Objects: {:.1} MB ({} files verified)", stats.assets_mb, stats.assets_count));
        self.libs_label.set_text(&format!("• Libraries: {:.1} MB ({} jars & native libs)", stats.libraries_mb, stats.libraries_count));
        self.mods_label.set_text(&format!("• Fabric Mods: {:.1} MB ({} active/disabled mods)", stats.mods_mb, stats.mods_count));
        self.shaders_label.set_text(&format!("• Shaderpacks: {:.1} MB ({} shader packs installed)", stats.shaders_mb, stats.shaders_count));
        self.logs_label.set_text(&format!("• Logs & Crash Dumps: {:.1} MB ({} logs, {} crash dumps)", stats.logs_mb, stats.logs_count, stats.crash_reports_count));
    }
}
