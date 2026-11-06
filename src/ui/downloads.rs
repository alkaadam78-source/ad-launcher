use crate::core::manifest::VersionEntry;
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, Label, ListBox, ListBoxRow, Orientation, ProgressBar, ScrolledWindow,
};
use std::rc::Rc;

pub struct DownloadsView {
    pub container: GtkBox,
    pub status_label: Label,
    pub progress_bar: ProgressBar,
    pub download_1_20_1_btn: Button,
    pub fetch_manifest_btn: Button,
    pub versions_list: ListBox,
}

impl DownloadsView {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 16);
        container.set_css_classes(&["content-area"]);

        let title = Label::new(Some("DOWNLOAD MANAGER"));
        title.set_css_classes(&["page-title"]);
        title.set_halign(gtk4::Align::Start);

        let subtitle = Label::new(Some("Official Mojang manifest engine and game file installer"));
        subtitle.set_css_classes(&["page-subtitle"]);
        subtitle.set_halign(gtk4::Align::Start);

        container.append(&title);
        container.append(&subtitle);

        // Quick 1.20.1 Download Action Card
        let action_card = GtkBox::new(Orientation::Vertical, 12);
        action_card.set_css_classes(&["card"]);

        let card_title = Label::new(Some("TARGET VERSION: MINECRAFT 1.20.1"));
        card_title.set_css_classes(&["card-title"]);
        card_title.set_halign(gtk4::Align::Start);
        action_card.append(&card_title);

        let card_desc = Label::new(Some(
            "Verifies and downloads official 1.20.1 client jar, asset index, required sound/texture assets, and Linux libraries with SHA-1 checksum validation."
        ));
        card_desc.set_css_classes(&["page-subtitle"]);
        card_desc.set_wrap(true);
        card_desc.set_halign(gtk4::Align::Start);
        action_card.append(&card_desc);

        let download_1_20_1_btn = Button::with_label("⬇  DOWNLOAD MINECRAFT 1.20.1");
        download_1_20_1_btn.set_css_classes(&["btn-play"]);
        action_card.append(&download_1_20_1_btn);

        let progress_bar = ProgressBar::new();
        progress_bar.set_fraction(0.0);
        progress_bar.set_visible(false);
        action_card.append(&progress_bar);

        let status_label = Label::new(Some("Ready to check files."));
        status_label.set_css_classes(&["badge-ready"]);
        status_label.set_halign(gtk4::Align::Start);
        action_card.append(&status_label);

        container.append(&action_card);

        // Version Manifest Explorer Card
        let manifest_card = GtkBox::new(Orientation::Vertical, 10);
        manifest_card.set_css_classes(&["card"]);
        manifest_card.set_vexpand(true);

        let header_row = GtkBox::new(Orientation::Horizontal, 12);
        let m_title = Label::new(Some("MOJANG VERSION MANIFEST"));
        m_title.set_css_classes(&["card-title"]);
        m_title.set_hexpand(true);
        m_title.set_halign(gtk4::Align::Start);
        header_row.append(&m_title);

        let fetch_manifest_btn = Button::with_label("↻ Fetch Manifest");
        fetch_manifest_btn.set_css_classes(&["btn-secondary"]);
        header_row.append(&fetch_manifest_btn);

        manifest_card.append(&header_row);

        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);
        let versions_list = ListBox::new();
        versions_list.set_selection_mode(gtk4::SelectionMode::None);
        scrolled.set_child(Some(&versions_list));

        manifest_card.append(&scrolled);
        container.append(&manifest_card);

        Self {
            container,
            status_label,
            progress_bar,
            download_1_20_1_btn,
            fetch_manifest_btn,
            versions_list,
        }
    }

    pub fn set_progress(&self, fraction: f64, status: &str) {
        self.progress_bar.set_visible(true);
        self.progress_bar.set_fraction(fraction);
        self.status_label.set_text(status);
    }

    pub fn populate_versions(&self, versions: &[VersionEntry], on_select: Rc<dyn Fn(String, String)>) {
        while let Some(child) = self.versions_list.first_child() {
            self.versions_list.remove(&child);
        }

        for v in versions.iter().take(50) {
            let row = ListBoxRow::new();
            let row_box = GtkBox::new(Orientation::Horizontal, 12);
            row_box.set_margin_top(6);
            row_box.set_margin_bottom(6);
            row_box.set_margin_start(12);
            row_box.set_margin_end(12);

            let id_lbl = Label::new(Some(&v.id));
            id_lbl.set_css_classes(&["card-title"]);
            id_lbl.set_halign(gtk4::Align::Start);
            row_box.append(&id_lbl);

            let type_lbl = Label::new(Some(&v.version_type));
            if v.version_type == "release" {
                type_lbl.set_css_classes(&["badge-ready"]);
            } else {
                type_lbl.set_css_classes(&["badge-info"]);
            }
            row_box.append(&type_lbl);

            let time_lbl = Label::new(Some(&v.release_time[..10]));
            time_lbl.set_css_classes(&["page-subtitle"]);
            time_lbl.set_hexpand(true);
            time_lbl.set_halign(gtk4::Align::Start);
            row_box.append(&time_lbl);

            let btn = Button::with_label("Install");
            btn.set_css_classes(&["btn-secondary"]);
            let id = v.id.clone();
            let url = v.url.clone();
            let cb = on_select.clone();
            btn.connect_clicked(move |_| {
                cb(id.clone(), url.clone());
            });
            row_box.append(&btn);

            row.set_child(Some(&row_box));
            self.versions_list.append(&row);
        }
    }
}
