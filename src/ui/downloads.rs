use crate::core::manifest::VersionEntry;
use crate::ui::{create_empty_state, create_page_header, create_section_divider};
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, Entry, Label, ListBox, ListBoxRow, Orientation, ProgressBar,
    ScrolledWindow,
};
use std::cell::RefCell;
use std::rc::Rc;

pub struct DownloadsView {
    pub container: GtkBox,
    pub status_label: Label,
    pub progress_bar: ProgressBar,
    pub download_1_20_1_btn: Button,
    pub quick_1_21_4_btn: Button,
    pub quick_1_16_5_btn: Button,
    pub quick_1_12_2_btn: Button,
    pub quick_1_8_9_btn: Button,
    pub fetch_manifest_btn: Button,
    pub search_entry: Entry,
    pub filter_releases_btn: Button,
    pub filter_snapshots_btn: Button,
    pub filter_historical_btn: Button,
    pub filter_all_btn: Button,
    pub versions_list: ListBox,
    all_versions: Rc<RefCell<Vec<VersionEntry>>>,
    active_filter: Rc<RefCell<String>>,
    install_callback: Rc<RefCell<Option<Rc<dyn Fn(String, String)>>>>,
}

impl DownloadsView {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 18);
        container.set_css_classes(&["content-area"]);

        // Page Header (Rule 1: Heading with subtle ornamental separator)
        container.append(&create_page_header(
            "ROYAL REPOSITORY & VERSION ARCHIVE",
            "Complete official Minecraft catalog from 1.0 to 1.21.4+, Snapshots, & Historical Alpha/Beta eras",
        ));

        // 1. Quick Curated Editions Action Card
        let quick_card = GtkBox::new(Orientation::Vertical, 12);
        quick_card.set_css_classes(&["card"]);

        let q_title = Label::new(Some("CURATED HISTORICAL EDITIONS (1-CLICK COMMISSION)"));
        q_title.set_css_classes(&["card-title"]);
        q_title.set_halign(gtk4::Align::Start);
        quick_card.append(&q_title);

        let q_desc = Label::new(Some(
            "Commission and configure verified instance profiles for favored epochs of the Minecraft continuum."
        ));
        q_desc.set_css_classes(&["page-subtitle"]);
        q_desc.set_wrap(true);
        q_desc.set_halign(gtk4::Align::Start);
        quick_card.append(&q_desc);

        let quick_row = GtkBox::new(Orientation::Horizontal, 10);
        quick_row.set_hexpand(true);

        let quick_1_21_4_btn = Button::with_label("👑 1.21.4 (Modern Epoch)");
        quick_1_21_4_btn.set_css_classes(&["btn-cyan"]);
        quick_1_21_4_btn.set_hexpand(true);

        let download_1_20_1_btn = Button::with_label("⚔ 1.20.1 (Renaissance Fabric)");
        download_1_20_1_btn.set_css_classes(&["btn-play"]);
        download_1_20_1_btn.set_hexpand(true);

        let quick_1_16_5_btn = Button::with_label("🌲 1.16.5 (Nether Epoch)");
        quick_1_16_5_btn.set_css_classes(&["btn-emerald"]);
        quick_1_16_5_btn.set_hexpand(true);

        let quick_1_12_2_btn = Button::with_label("⚙ 1.12.2 (Golden Forge)");
        quick_1_12_2_btn.set_css_classes(&["btn-amber"]);
        quick_1_12_2_btn.set_hexpand(true);

        let quick_1_8_9_btn = Button::with_label("🛡 1.8.9 (Combat Classic)");
        quick_1_8_9_btn.set_css_classes(&["btn-purple"]);
        quick_1_8_9_btn.set_hexpand(true);

        quick_row.append(&quick_1_21_4_btn);
        quick_row.append(&download_1_20_1_btn);
        quick_row.append(&quick_1_16_5_btn);
        quick_row.append(&quick_1_12_2_btn);
        quick_row.append(&quick_1_8_9_btn);
        quick_card.append(&quick_row);

        // Progress bar
        let progress_bar = ProgressBar::new();
        progress_bar.set_fraction(0.0);
        progress_bar.set_visible(false);
        quick_card.append(&progress_bar);

        let status_label = Label::new(Some(
            "Repository Status: Ready to retrieve archives. 60+ versions cataloged below.",
        ));
        status_label.set_css_classes(&["badge-ready"]);
        status_label.set_halign(gtk4::Align::Start);
        quick_card.append(&status_label);

        container.append(&quick_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // 2. Full Mojang Version Catalog Card
        let catalog_card = GtkBox::new(Orientation::Vertical, 12);
        catalog_card.set_css_classes(&["card"]);
        catalog_card.set_vexpand(true);

        let cat_header = GtkBox::new(Orientation::Horizontal, 12);
        let m_title = Label::new(Some("OFFICIAL MANIFEST & VERSION CATALOG"));
        m_title.set_css_classes(&["card-title"]);
        m_title.set_hexpand(true);
        m_title.set_halign(gtk4::Align::Start);
        cat_header.append(&m_title);

        let fetch_manifest_btn = Button::with_label("↻ Check Online Manifest");
        fetch_manifest_btn.set_css_classes(&["btn-amber"]);
        cat_header.append(&fetch_manifest_btn);
        catalog_card.append(&cat_header);

        // Search & Filter Row
        let filter_row = GtkBox::new(Orientation::Horizontal, 8);
        filter_row.set_margin_top(4);
        filter_row.set_margin_bottom(6);

        let search_entry = Entry::new();
        search_entry.set_placeholder_text(Some(
            "Search all versions: 1.21, 1.20.1, 1.16.5, 1.12.2, 1.8.9, b1.7.3...",
        ));
        search_entry.set_hexpand(true);
        filter_row.append(&search_entry);

        let filter_releases_btn = Button::with_label("👑 Releases (All)");
        filter_releases_btn.set_css_classes(&["filter-btn", "active"]);

        let filter_snapshots_btn = Button::with_label("📸 Snapshots");
        filter_snapshots_btn.set_css_classes(&["filter-btn"]);

        let filter_historical_btn = Button::with_label("📜 Historical Beta/Alpha");
        filter_historical_btn.set_css_classes(&["filter-btn"]);

        let filter_all_btn = Button::with_label("🌐 All");
        filter_all_btn.set_css_classes(&["filter-btn"]);

        filter_row.append(&filter_releases_btn);
        filter_row.append(&filter_snapshots_btn);
        filter_row.append(&filter_historical_btn);
        filter_row.append(&filter_all_btn);
        catalog_card.append(&filter_row);

        // Scrolled list of all Minecraft versions
        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);
        scrolled.set_min_content_height(450);

        let versions_list = ListBox::new();
        versions_list.set_selection_mode(gtk4::SelectionMode::None);
        scrolled.set_child(Some(&versions_list));

        catalog_card.append(&scrolled);
        container.append(&catalog_card);

        let initial_versions = default_prepopulated_versions();
        let all_versions = Rc::new(RefCell::new(initial_versions));
        let active_filter = Rc::new(RefCell::new("release".to_string()));
        let install_callback: Rc<RefCell<Option<Rc<dyn Fn(String, String)>>>> =
            Rc::new(RefCell::new(None));

        let view = Self {
            container,
            status_label,
            progress_bar,
            download_1_20_1_btn,
            quick_1_21_4_btn,
            quick_1_16_5_btn,
            quick_1_12_2_btn,
            quick_1_8_9_btn,
            fetch_manifest_btn,
            search_entry,
            filter_releases_btn,
            filter_snapshots_btn,
            filter_historical_btn,
            filter_all_btn,
            versions_list,
            all_versions,
            active_filter,
            install_callback,
        };

        view.setup_internal_filters();
        view
    }

    fn setup_internal_filters(&self) {
        let all_ver = self.all_versions.clone();
        let act_filter = self.active_filter.clone();
        let list = self.versions_list.clone();
        let cb = self.install_callback.clone();
        let search_ent = self.search_entry.clone();

        let do_refresh = move || {
            let filter = act_filter.borrow().clone();
            let query = search_ent
                .text()
                .to_string()
                .to_lowercase()
                .trim()
                .to_string();
            let versions = all_ver.borrow();
            let callback = cb.borrow().clone();

            while let Some(child) = list.first_child() {
                list.remove(&child);
            }

            let filtered: Vec<&VersionEntry> = versions
                .iter()
                .filter(|v| {
                    let matches_search = query.is_empty() || v.id.to_lowercase().contains(&query);
                    let matches_type = match filter.as_str() {
                        "release" => v.version_type == "release",
                        "snapshot" => v.version_type == "snapshot",
                        "historical" => {
                            v.version_type == "old_beta" || v.version_type == "old_alpha"
                        }
                        _ => true,
                    };
                    matches_search && matches_type
                })
                .take(250)
                .collect();

            if filtered.is_empty() {
                let row = ListBoxRow::new();
                let empty_widget = create_empty_state(
                    "📥",
                    "No Versions Found",
                    "No Minecraft versions found matching the current search criteria.",
                );
                row.set_child(Some(&empty_widget));
                list.append(&row);
                return;
            }

            for v in filtered {
                let row = ListBoxRow::new();
                let row_box = GtkBox::new(Orientation::Horizontal, 12);
                row_box.set_css_classes(&["card"]);
                row_box.set_margin_bottom(6);

                let id_lbl = Label::new(Some(&v.id));
                id_lbl.set_css_classes(&["card-title"]);
                id_lbl.set_halign(gtk4::Align::Start);
                row_box.append(&id_lbl);

                let type_lbl = Label::new(Some(&v.version_type));
                if v.version_type == "release" {
                    type_lbl.set_css_classes(&["badge-ready"]);
                } else if v.version_type == "snapshot" {
                    type_lbl.set_css_classes(&["badge-info"]);
                } else {
                    type_lbl.set_css_classes(&["badge-turbo"]);
                }
                row_box.append(&type_lbl);

                let date_str = if v.release_time.len() >= 10 {
                    &v.release_time[..10]
                } else {
                    &v.release_time
                };
                let time_lbl = Label::new(Some(date_str));
                time_lbl.set_css_classes(&["page-subtitle"]);
                time_lbl.set_hexpand(true);
                time_lbl.set_halign(gtk4::Align::Start);
                row_box.append(&time_lbl);

                let btn = Button::with_label("📥 Install & Play");
                btn.set_css_classes(&["btn-emerald"]);
                let id = v.id.clone();
                let url = v.url.clone();
                let cb_clone = callback.clone();
                btn.connect_clicked(move |_| {
                    if let Some(ref f) = cb_clone {
                        f(id.clone(), url.clone());
                    }
                });
                row_box.append(&btn);

                row.set_child(Some(&row_box));
                list.append(&row);
            }
        };

        let ref_fn = Rc::new(do_refresh);

        ref_fn();

        {
            let r = ref_fn.clone();
            self.search_entry.connect_changed(move |_| {
                r();
            });
        }

        let filters = [
            (&self.filter_releases_btn, "release"),
            (&self.filter_snapshots_btn, "snapshot"),
            (&self.filter_historical_btn, "historical"),
            (&self.filter_all_btn, "all"),
        ];

        let btns = [
            self.filter_releases_btn.clone(),
            self.filter_snapshots_btn.clone(),
            self.filter_historical_btn.clone(),
            self.filter_all_btn.clone(),
        ];

        for (btn, key) in filters {
            let act = self.active_filter.clone();
            let r = ref_fn.clone();
            let all_b = btns.clone();
            let b_clone = btn.clone();
            let k = key.to_string();
            btn.connect_clicked(move |_| {
                *act.borrow_mut() = k.clone();
                for b in &all_b {
                    b.remove_css_class("active");
                }
                b_clone.add_css_class("active");
                r();
            });
        }
    }

    pub fn set_progress(&self, fraction: f64, status: &str) {
        self.progress_bar.set_visible(true);
        self.progress_bar.set_fraction(fraction);
        self.status_label.set_text(status);
        if fraction >= 1.0 {
            self.status_label.set_css_classes(&["badge-ready"]);
        } else if fraction > 0.0 {
            self.status_label.set_css_classes(&["badge-info"]);
        }
    }

    pub fn populate_versions(
        &self,
        versions: &[VersionEntry],
        on_select: Rc<dyn Fn(String, String)>,
    ) {
        let mut list = versions.to_vec();
        if list.is_empty() {
            list = default_prepopulated_versions();
        }
        *self.all_versions.borrow_mut() = list;
        *self.install_callback.borrow_mut() = Some(on_select);

        let search_text = self.search_entry.text();
        self.search_entry.set_text(&search_text);
    }

    pub fn get_version_entry(&self, version_id: &str) -> Option<VersionEntry> {
        self.all_versions
            .borrow()
            .iter()
            .find(|v| v.id == version_id)
            .cloned()
            .or_else(|| {
                Some(VersionEntry {
                    id: version_id.to_string(),
                    version_type: "release".to_string(),
                    url: format!(
                        "https://piston-meta.mojang.com/v1/packages/{}.json",
                        version_id
                    ),
                    time: String::new(),
                    release_time: String::new(),
                    sha1: String::new(),
                })
            })
    }
}

fn default_prepopulated_versions() -> Vec<VersionEntry> {
    let releases = [
        ("1.21.4", "2024-12-03"),
        ("1.21.3", "2024-10-23"),
        ("1.21.2", "2024-10-22"),
        ("1.21.1", "2024-08-08"),
        ("1.21", "2024-06-13"),
        ("1.20.6", "2024-04-29"),
        ("1.20.5", "2024-04-23"),
        ("1.20.4", "2023-12-07"),
        ("1.20.3", "2023-12-05"),
        ("1.20.2", "2023-09-21"),
        ("1.20.1", "2023-06-12"),
        ("1.20", "2023-06-07"),
        ("1.19.4", "2023-03-14"),
        ("1.19.3", "2022-12-07"),
        ("1.19.2", "2022-08-05"),
        ("1.19.1", "2022-07-27"),
        ("1.19", "2022-06-07"),
        ("1.18.2", "2022-02-28"),
        ("1.18.1", "2021-12-10"),
        ("1.18", "2021-11-30"),
        ("1.17.1", "2021-07-06"),
        ("1.17", "2021-06-08"),
        ("1.16.5", "2021-01-15"),
        ("1.16.4", "2020-11-02"),
        ("1.16.3", "2020-09-10"),
        ("1.16.2", "2020-08-11"),
        ("1.16.1", "2020-06-24"),
        ("1.16", "2020-06-23"),
        ("1.15.2", "2020-01-21"),
        ("1.15.1", "2019-12-16"),
        ("1.15", "2019-12-10"),
        ("1.14.4", "2019-07-19"),
        ("1.14.3", "2019-06-24"),
        ("1.14.2", "2019-05-27"),
        ("1.14.1", "2019-05-13"),
        ("1.14", "2019-04-23"),
        ("1.13.2", "2018-10-22"),
        ("1.13.1", "2018-08-22"),
        ("1.13", "2018-07-18"),
        ("1.12.2", "2017-09-18"),
        ("1.12.1", "2017-08-03"),
        ("1.12", "2017-06-07"),
        ("1.11.2", "2016-12-21"),
        ("1.11", "2016-11-14"),
        ("1.10.2", "2016-06-23"),
        ("1.10", "2016-06-08"),
        ("1.9.4", "2016-05-10"),
        ("1.9.2", "2016-03-30"),
        ("1.9", "2016-02-29"),
        ("1.8.9", "2015-12-09"),
        ("1.8.8", "2015-07-28"),
        ("1.8", "2014-09-02"),
        ("1.7.10", "2014-06-26"),
        ("1.7.9", "2014-04-14"),
        ("1.7.2", "2013-10-25"),
        ("1.6.4", "2013-09-19"),
        ("1.5.2", "2013-05-02"),
        ("1.4.7", "2013-01-09"),
        ("1.3.2", "2012-08-16"),
        ("1.2.5", "2012-04-04"),
        ("1.1", "2012-01-12"),
        ("1.0", "2011-11-18"),
    ];

    let mut list = Vec::new();
    for (id, date) in releases {
        list.push(VersionEntry {
            id: id.to_string(),
            version_type: "release".to_string(),
            url: format!("https://piston-meta.mojang.com/v1/packages/{}.json", id),
            time: format!("{}T12:00:00+00:00", date),
            release_time: format!("{}T12:00:00+00:00", date),
            sha1: String::new(),
        });
    }

    let snapshots = [
        ("24w45a", "2024-11-06"),
        ("24w44a", "2024-10-30"),
        ("24w40a", "2024-10-02"),
        ("1.21-pre1", "2024-05-30"),
        ("1.20.5-pre1", "2024-04-10"),
    ];
    for (id, date) in snapshots {
        list.push(VersionEntry {
            id: id.to_string(),
            version_type: "snapshot".to_string(),
            url: format!("https://piston-meta.mojang.com/v1/packages/{}.json", id),
            time: format!("{}T12:00:00+00:00", date),
            release_time: format!("{}T12:00:00+00:00", date),
            sha1: String::new(),
        });
    }

    let retros = [
        ("b1.8.1", "2011-09-19"),
        ("b1.7.3", "2011-07-08"),
        ("b1.5_01", "2011-04-20"),
        ("a1.2.6", "2010-12-03"),
        ("c0.30", "2009-12-23"),
        ("rd-132211", "2009-05-13"),
    ];
    for (id, date) in retros {
        list.push(VersionEntry {
            id: id.to_string(),
            version_type: "old_beta".to_string(),
            url: format!("https://piston-meta.mojang.com/v1/packages/{}.json", id),
            time: format!("{}T12:00:00+00:00", date),
            release_time: format!("{}T12:00:00+00:00", date),
            sha1: String::new(),
        });
    }

    list
}
