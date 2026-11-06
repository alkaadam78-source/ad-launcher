use crate::core::mods::ModInfo;
use crate::integrations::modrinth::ModrinthProject;
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, CheckButton, Entry, Label, ListBox, ListBoxRow, Orientation,
    ScrolledWindow,
};
use std::rc::Rc;

pub struct ModsView {
    pub container: GtkBox,
    pub installed_list: ListBox,
    pub search_entry: Entry,
    pub search_btn: Button,
    pub search_results_list: ListBox,
    pub refresh_btn: Button,
    pub quick_sodium_btn: Button,
    pub quick_iris_btn: Button,
    pub quick_lithium_btn: Button,
    pub quick_ferrite_btn: Button,
}

impl ModsView {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 16);
        container.set_css_classes(&["content-area"]);

        let title = Label::new(Some("MOD MANAGER & MODRINTH INTEGRATION"));
        title.set_css_classes(&["page-title"]);
        title.set_halign(gtk4::Align::Start);

        let subtitle = Label::new(Some("Manage Fabric 1.20.1 performance optimizations and addons"));
        subtitle.set_css_classes(&["page-subtitle"]);
        subtitle.set_halign(gtk4::Align::Start);

        container.append(&title);
        container.append(&subtitle);

        // Quick Install Performance Pack Card
        let quick_card = GtkBox::new(Orientation::Vertical, 10);
        quick_card.set_css_classes(&["card"]);

        let quick_title = Label::new(Some("ESSENTIAL PERFORMANCE MODS (FABRIC 1.20.1)"));
        quick_title.set_css_classes(&["card-title"]);
        quick_title.set_halign(gtk4::Align::Start);
        quick_card.append(&quick_title);

        let quick_row = GtkBox::new(Orientation::Horizontal, 8);
        let quick_sodium_btn = Button::with_label("+ Sodium (Renderer)");
        quick_sodium_btn.set_css_classes(&["btn-secondary"]);

        let quick_iris_btn = Button::with_label("+ Iris (Shaders)");
        quick_iris_btn.set_css_classes(&["btn-secondary"]);

        let quick_lithium_btn = Button::with_label("+ Lithium (Physics/Tick)");
        quick_lithium_btn.set_css_classes(&["btn-secondary"]);

        let quick_ferrite_btn = Button::with_label("+ FerriteCore (Memory)");
        quick_ferrite_btn.set_css_classes(&["btn-secondary"]);

        quick_row.append(&quick_sodium_btn);
        quick_row.append(&quick_iris_btn);
        quick_row.append(&quick_lithium_btn);
        quick_row.append(&quick_ferrite_btn);

        quick_card.append(&quick_row);
        container.append(&quick_card);

        // Search Modrinth Card
        let search_card = GtkBox::new(Orientation::Vertical, 10);
        search_card.set_css_classes(&["card"]);

        let search_title = Label::new(Some("SEARCH MODRINTH FOR 1.20.1 FABRIC MODS"));
        search_title.set_css_classes(&["card-title"]);
        search_title.set_halign(gtk4::Align::Start);
        search_card.append(&search_title);

        let s_row = GtkBox::new(Orientation::Horizontal, 8);
        let search_entry = Entry::new();
        search_entry.set_placeholder_text(Some("Search Modrinth (e.g. Indium, Continuity, LambDynamicLights)"));
        search_entry.set_hexpand(true);
        s_row.append(&search_entry);

        let search_btn = Button::with_label("🔍 Search");
        search_btn.set_css_classes(&["btn-primary"]);
        s_row.append(&search_btn);
        search_card.append(&s_row);

        let results_scrolled = ScrolledWindow::new();
        results_scrolled.set_max_content_height(140);
        let search_results_list = ListBox::new();
        search_results_list.set_selection_mode(gtk4::SelectionMode::None);
        results_scrolled.set_child(Some(&search_results_list));
        search_card.append(&results_scrolled);

        container.append(&search_card);

        // Installed Mods Card
        let installed_card = GtkBox::new(Orientation::Vertical, 10);
        installed_card.set_css_classes(&["card"]);
        installed_card.set_vexpand(true);

        let inst_header = GtkBox::new(Orientation::Horizontal, 12);
        let inst_title = Label::new(Some("INSTALLED MODS IN ACTIVE INSTANCE"));
        inst_title.set_css_classes(&["card-title"]);
        inst_title.set_hexpand(true);
        inst_title.set_halign(gtk4::Align::Start);
        inst_header.append(&inst_title);

        let refresh_btn = Button::with_label("↻ Reload Mods");
        refresh_btn.set_css_classes(&["btn-secondary"]);
        inst_header.append(&refresh_btn);

        installed_card.append(&inst_header);

        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);
        let installed_list = ListBox::new();
        installed_list.set_selection_mode(gtk4::SelectionMode::None);
        scrolled.set_child(Some(&installed_list));

        installed_card.append(&scrolled);
        container.append(&installed_card);

        Self {
            container,
            installed_list,
            search_entry,
            search_btn,
            search_results_list,
            refresh_btn,
            quick_sodium_btn,
            quick_iris_btn,
            quick_lithium_btn,
            quick_ferrite_btn,
        }
    }

    pub fn populate_installed_mods(
        &self,
        mods: &[ModInfo],
        on_toggle: Rc<dyn Fn(ModInfo)>,
        on_delete: Rc<dyn Fn(ModInfo)>,
    ) {
        while let Some(child) = self.installed_list.first_child() {
            self.installed_list.remove(&child);
        }

        if mods.is_empty() {
            let empty_lbl = Label::new(Some("No mods installed yet. Use the search or quick buttons above."));
            empty_lbl.set_css_classes(&["page-subtitle"]);
            empty_lbl.set_margin_top(16);
            empty_lbl.set_margin_bottom(16);
            self.installed_list.append(&empty_lbl);
            return;
        }

        for m in mods {
            let row = ListBoxRow::new();
            let row_box = GtkBox::new(Orientation::Horizontal, 12);
            row_box.set_margin_top(6);
            row_box.set_margin_bottom(6);
            row_box.set_margin_start(12);
            row_box.set_margin_end(12);

            let check = CheckButton::new();
            check.set_active(m.enabled);
            let m_clone = m.clone();
            let toggle_cb = on_toggle.clone();
            check.connect_toggled(move |_| {
                toggle_cb(m_clone.clone());
            });
            row_box.append(&check);

            let info_box = GtkBox::new(Orientation::Vertical, 2);
            info_box.set_hexpand(true);

            let name_box = GtkBox::new(Orientation::Horizontal, 8);
            let name_lbl = Label::new(Some(&m.name));
            name_lbl.set_css_classes(&["card-title"]);
            name_lbl.set_halign(gtk4::Align::Start);
            name_box.append(&name_lbl);

            let ver_lbl = Label::new(Some(&format!("v{}", m.version)));
            ver_lbl.set_css_classes(&["badge-info"]);
            name_box.append(&ver_lbl);

            if m.is_performance_mod {
                let perf_badge = Label::new(Some("PERFORMANCE"));
                perf_badge.set_css_classes(&["badge-ready"]);
                name_box.append(&perf_badge);
            }

            info_box.append(&name_box);

            let desc_lbl = Label::new(Some(&m.description));
            desc_lbl.set_halign(gtk4::Align::Start);
            desc_lbl.set_css_classes(&["page-subtitle"]);
            desc_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
            desc_lbl.set_max_width_chars(60);
            info_box.append(&desc_lbl);

            row_box.append(&info_box);

            let del_btn = Button::with_label("✕");
            del_btn.set_css_classes(&["btn-danger"]);
            let m_clone_del = m.clone();
            let del_cb = on_delete.clone();
            del_btn.connect_clicked(move |_| {
                del_cb(m_clone_del.clone());
            });
            row_box.append(&del_btn);

            row.set_child(Some(&row_box));
            self.installed_list.append(&row);
        }
    }

    pub fn populate_search_results(
        &self,
        projects: &[ModrinthProject],
        on_install: Rc<dyn Fn(String, String)>, // slug, title
    ) {
        while let Some(child) = self.search_results_list.first_child() {
            self.search_results_list.remove(&child);
        }

        if projects.is_empty() {
            let empty = Label::new(Some("No results found."));
            empty.set_css_classes(&["page-subtitle"]);
            self.search_results_list.append(&empty);
            return;
        }

        for p in projects {
            let row = ListBoxRow::new();
            let row_box = GtkBox::new(Orientation::Horizontal, 12);
            row_box.set_margin_top(4);
            row_box.set_margin_bottom(4);
            row_box.set_margin_start(8);
            row_box.set_margin_end(8);

            let info_box = GtkBox::new(Orientation::Vertical, 2);
            info_box.set_hexpand(true);

            let title_lbl = Label::new(Some(&p.title));
            title_lbl.set_css_classes(&["card-title"]);
            title_lbl.set_halign(gtk4::Align::Start);
            info_box.append(&title_lbl);

            let desc_lbl = Label::new(Some(&p.description));
            desc_lbl.set_css_classes(&["page-subtitle"]);
            desc_lbl.set_halign(gtk4::Align::Start);
            desc_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
            info_box.append(&desc_lbl);

            row_box.append(&info_box);

            let inst_btn = Button::with_label("Install");
            inst_btn.set_css_classes(&["btn-primary"]);
            let slug = p.slug.clone();
            let title = p.title.clone();
            let inst_cb = on_install.clone();
            inst_btn.connect_clicked(move |_| {
                inst_cb(slug.clone(), title.clone());
            });
            row_box.append(&inst_btn);

            row.set_child(Some(&row_box));
            self.search_results_list.append(&row);
        }
    }
}
