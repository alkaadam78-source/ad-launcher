use crate::core::mods::ModInfo;
use crate::ui::{create_empty_state, create_page_header, create_section_divider};
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, CheckButton, Entry, Label, ListBox, ListBoxRow, Orientation,
    ScrolledWindow,
};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct UnifiedModItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub author: String,
    pub source: String, // "Modrinth" or "CurseForge"
    pub download_url: Option<String>,
}

pub struct ModsView {
    pub container: GtkBox,
    pub installed_list: ListBox,
    pub search_entry: Entry,
    pub search_btn: Button,
    pub search_results_list: ListBox,
    pub refresh_btn: Button,
    pub version_entry: Entry,

    // Source buttons
    pub btn_source_modrinth: Button,
    pub btn_source_curseforge: Button,

    // Loader buttons
    pub btn_loader_fabric: Button,
    pub btn_loader_forge: Button,
    pub btn_loader_neoforge: Button,
    pub btn_loader_quilt: Button,

    // Quick Essential Mods buttons
    pub quick_sodium_btn: Button,
    pub quick_iris_btn: Button,
    pub quick_lithium_btn: Button,
    pub quick_ferrite_btn: Button,
    pub quick_jei_btn: Button,
    pub quick_appleskin_btn: Button,

    active_source: Rc<RefCell<String>>,
    active_loader: Rc<RefCell<String>>,
}

impl ModsView {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 18);
        container.set_css_classes(&["content-area"]);

        // Page Header (Rule 1: Heading with subtle ornamental separator)
        container.append(&create_page_header(
            "THE ILLUMINATED MOD ARCHIVE",
            "Curate and install modifications from Modrinth & CurseForge for Fabric, Forge, NeoForge, and Quilt",
        ));

        // 1-Click Essential Performance Suite Card
        let quick_card = GtkBox::new(Orientation::Vertical, 12);
        quick_card.set_css_classes(&["card"]);

        let quick_title = Label::new(Some("ESSENTIAL PERFORMANCE SUITE (1-CLICK COMMISSION)"));
        quick_title.set_css_classes(&["card-title"]);
        quick_title.set_halign(gtk4::Align::Start);
        quick_card.append(&quick_title);

        let quick_row = GtkBox::new(Orientation::Horizontal, 8);
        let quick_sodium_btn = Button::with_label("+ Sodium / Embeddium");
        quick_sodium_btn.set_css_classes(&["btn-cyan"]);

        let quick_iris_btn = Button::with_label("+ Iris / Oculus");
        quick_iris_btn.set_css_classes(&["btn-purple"]);

        let quick_lithium_btn = Button::with_label("+ Lithium");
        quick_lithium_btn.set_css_classes(&["btn-emerald"]);

        let quick_ferrite_btn = Button::with_label("+ FerriteCore");
        quick_ferrite_btn.set_css_classes(&["btn-amber"]);

        let quick_jei_btn = Button::with_label("+ JEI / Items");
        quick_jei_btn.set_css_classes(&["btn-secondary"]);

        let quick_appleskin_btn = Button::with_label("+ AppleSkin");
        quick_appleskin_btn.set_css_classes(&["btn-secondary"]);

        quick_row.append(&quick_sodium_btn);
        quick_row.append(&quick_iris_btn);
        quick_row.append(&quick_lithium_btn);
        quick_row.append(&quick_ferrite_btn);
        quick_row.append(&quick_jei_btn);
        quick_row.append(&quick_appleskin_btn);

        quick_card.append(&quick_row);
        container.append(&quick_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // Search Modrinth & CurseForge Card
        let search_card = GtkBox::new(Orientation::Vertical, 12);
        search_card.set_css_classes(&["card"]);

        let search_title = Label::new(Some("SEARCH ONLINE REPOSITORIES (MODRINTH & CURSEFORGE)"));
        search_title.set_css_classes(&["card-title"]);
        search_title.set_halign(gtk4::Align::Start);
        search_card.append(&search_title);

        // Controls Row: Source and Loaders
        let controls_row = GtkBox::new(Orientation::Horizontal, 8);

        let src_lbl = Label::new(Some("Archive:"));
        src_lbl.set_css_classes(&["page-subtitle"]);
        controls_row.append(&src_lbl);

        let btn_source_modrinth = Button::with_label("🌐 Modrinth");
        btn_source_modrinth.set_css_classes(&["filter-btn", "active"]);
        controls_row.append(&btn_source_modrinth);

        let btn_source_curseforge = Button::with_label("🔥 CurseForge");
        btn_source_curseforge.set_css_classes(&["filter-btn"]);
        controls_row.append(&btn_source_curseforge);

        let sep = Label::new(Some("| Loader:"));
        sep.set_css_classes(&["page-subtitle"]);
        sep.set_margin_start(8);
        controls_row.append(&sep);

        let btn_loader_fabric = Button::with_label("Fabric");
        btn_loader_fabric.set_css_classes(&["filter-btn", "active"]);
        controls_row.append(&btn_loader_fabric);

        let btn_loader_forge = Button::with_label("Forge");
        btn_loader_forge.set_css_classes(&["filter-btn"]);
        controls_row.append(&btn_loader_forge);

        let btn_loader_neoforge = Button::with_label("NeoForge");
        btn_loader_neoforge.set_css_classes(&["filter-btn"]);
        controls_row.append(&btn_loader_neoforge);

        let btn_loader_quilt = Button::with_label("Quilt");
        btn_loader_quilt.set_css_classes(&["filter-btn"]);
        controls_row.append(&btn_loader_quilt);

        let ver_lbl = Label::new(Some("| Version:"));
        ver_lbl.set_css_classes(&["page-subtitle"]);
        ver_lbl.set_margin_start(8);
        controls_row.append(&ver_lbl);

        let version_entry = Entry::new();
        version_entry.set_text("1.20.1");
        version_entry.set_width_chars(8);
        controls_row.append(&version_entry);

        search_card.append(&controls_row);

        // Search Input Row
        let s_row = GtkBox::new(Orientation::Horizontal, 8);
        s_row.set_margin_top(4);

        let search_entry = Entry::new();
        search_entry.set_placeholder_text(Some(
            "Search modifications (e.g. JEI, Waystones, Twilight Forest, Sodium)...",
        ));
        search_entry.set_hexpand(true);
        s_row.append(&search_entry);

        let search_btn = Button::with_label("🔍 Search Archive");
        search_btn.set_css_classes(&["btn-cyan"]);
        s_row.append(&search_btn);
        search_card.append(&s_row);

        let results_scrolled = ScrolledWindow::new();
        results_scrolled.set_min_content_height(200);
        results_scrolled.set_vexpand(true);

        let search_results_list = ListBox::new();
        search_results_list.set_selection_mode(gtk4::SelectionMode::None);
        results_scrolled.set_child(Some(&search_results_list));
        search_card.append(&results_scrolled);

        container.append(&search_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // Installed Mods Card
        let installed_card = GtkBox::new(Orientation::Vertical, 12);
        installed_card.set_css_classes(&["card"]);
        installed_card.set_vexpand(true);

        let inst_header = GtkBox::new(Orientation::Horizontal, 12);
        let inst_title = Label::new(Some("ACTIVE INSTANCE MOD ARCHIVE"));
        inst_title.set_css_classes(&["card-title"]);
        inst_title.set_hexpand(true);
        inst_title.set_halign(gtk4::Align::Start);
        inst_header.append(&inst_title);

        let refresh_btn = Button::with_label("↻ Reload Archive");
        refresh_btn.set_css_classes(&["btn-amber"]);
        inst_header.append(&refresh_btn);

        installed_card.append(&inst_header);

        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);
        scrolled.set_min_content_height(180);

        let installed_list = ListBox::new();
        installed_list.set_selection_mode(gtk4::SelectionMode::None);
        scrolled.set_child(Some(&installed_list));

        installed_card.append(&scrolled);
        container.append(&installed_card);

        let active_source = Rc::new(RefCell::new("modrinth".to_string()));
        let active_loader = Rc::new(RefCell::new("fabric".to_string()));

        let view = Self {
            container,
            installed_list,
            search_entry,
            search_btn,
            search_results_list,
            refresh_btn,
            version_entry,
            btn_source_modrinth,
            btn_source_curseforge,
            btn_loader_fabric,
            btn_loader_forge,
            btn_loader_neoforge,
            btn_loader_quilt,
            quick_sodium_btn,
            quick_iris_btn,
            quick_lithium_btn,
            quick_ferrite_btn,
            quick_jei_btn,
            quick_appleskin_btn,
            active_source,
            active_loader,
        };

        view.setup_source_and_loader_buttons();
        view
    }

    fn setup_source_and_loader_buttons(&self) {
        // Source buttons toggling
        let sources = [
            (&self.btn_source_modrinth, "modrinth"),
            (&self.btn_source_curseforge, "curseforge"),
        ];
        let src_btns = [
            self.btn_source_modrinth.clone(),
            self.btn_source_curseforge.clone(),
        ];

        for (btn, key) in sources {
            let act = self.active_source.clone();
            let all = src_btns.clone();
            let b_clone = btn.clone();
            let k = key.to_string();
            btn.connect_clicked(move |_| {
                *act.borrow_mut() = k.clone();
                for b in &all {
                    b.remove_css_class("active");
                }
                b_clone.add_css_class("active");
            });
        }

        // Loader buttons toggling
        let loaders = [
            (&self.btn_loader_fabric, "fabric"),
            (&self.btn_loader_forge, "forge"),
            (&self.btn_loader_neoforge, "neoforge"),
            (&self.btn_loader_quilt, "quilt"),
        ];
        let ldr_btns = [
            self.btn_loader_fabric.clone(),
            self.btn_loader_forge.clone(),
            self.btn_loader_neoforge.clone(),
            self.btn_loader_quilt.clone(),
        ];

        for (btn, key) in loaders {
            let act = self.active_loader.clone();
            let all = ldr_btns.clone();
            let b_clone = btn.clone();
            let k = key.to_string();
            btn.connect_clicked(move |_| {
                *act.borrow_mut() = k.clone();
                for b in &all {
                    b.remove_css_class("active");
                }
                b_clone.add_css_class("active");
            });
        }
    }

    pub fn get_active_source(&self) -> String {
        self.active_source.borrow().clone()
    }

    pub fn get_active_loader(&self) -> String {
        self.active_loader.borrow().clone()
    }

    pub fn get_active_version(&self) -> String {
        let text = self.version_entry.text().to_string();
        if text.trim().is_empty() {
            "1.20.1".to_string()
        } else {
            text.trim().to_string()
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
            let row = ListBoxRow::new();
            let empty_widget = create_empty_state(
                "🧩",
                "No Active Modifications",
                "No modifications present in current world archive. Search above to install.",
            );
            row.set_child(Some(&empty_widget));
            self.installed_list.append(&row);
            return;
        }

        for m in mods {
            let row = ListBoxRow::new();
            let row_box = GtkBox::new(Orientation::Horizontal, 12);
            row_box.set_css_classes(&["card"]);
            row_box.set_margin_bottom(6);

            let check = CheckButton::new();
            check.set_active(m.enabled);
            let m_clone_toggle = m.clone();
            let toggle_cb = on_toggle.clone();
            check.connect_toggled(move |_| {
                toggle_cb(m_clone_toggle.clone());
            });
            row_box.append(&check);

            let info_box = GtkBox::new(Orientation::Vertical, 2);
            info_box.set_hexpand(true);

            let title_box = GtkBox::new(Orientation::Horizontal, 8);
            let name_lbl = Label::new(Some(&m.name));
            name_lbl.set_css_classes(&["card-title"]);
            name_lbl.set_halign(gtk4::Align::Start);
            title_box.append(&name_lbl);

            let ver_lbl = Label::new(Some(&m.version));
            ver_lbl.set_css_classes(&["badge-ready"]);
            title_box.append(&ver_lbl);
            info_box.append(&title_box);

            let desc_lbl = Label::new(Some(&m.description));
            desc_lbl.set_halign(gtk4::Align::Start);
            desc_lbl.set_css_classes(&["page-subtitle"]);
            desc_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
            desc_lbl.set_max_width_chars(60);
            info_box.append(&desc_lbl);

            row_box.append(&info_box);

            let del_btn = Button::with_label("✕ Remove");
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

    pub fn populate_unified_search_results(
        &self,
        mods: &[UnifiedModItem],
        on_install: Rc<dyn Fn(UnifiedModItem)>,
    ) {
        while let Some(child) = self.search_results_list.first_child() {
            self.search_results_list.remove(&child);
        }

        if mods.is_empty() {
            let row = ListBoxRow::new();
            let empty_widget = create_empty_state(
                "🔍",
                "No Modifications Found",
                "Try adjusting keywords, repository source, or loader.",
            );
            row.set_child(Some(&empty_widget));
            self.search_results_list.append(&row);
            return;
        }

        for p in mods {
            let row = ListBoxRow::new();
            let row_box = GtkBox::new(Orientation::Horizontal, 12);
            row_box.set_css_classes(&["card"]);
            row_box.set_margin_bottom(6);

            let info_box = GtkBox::new(Orientation::Vertical, 2);
            info_box.set_hexpand(true);

            let header_row = GtkBox::new(Orientation::Horizontal, 8);
            let title_lbl = Label::new(Some(&p.title));
            title_lbl.set_css_classes(&["card-title"]);
            title_lbl.set_halign(gtk4::Align::Start);
            header_row.append(&title_lbl);

            let src_badge = Label::new(Some(&p.source));
            if p.source == "Modrinth" {
                src_badge.set_css_classes(&["badge-ready"]);
            } else {
                src_badge.set_css_classes(&["badge-warning"]);
            }
            header_row.append(&src_badge);

            let author_lbl = Label::new(Some(&format!("by {}", p.author)));
            author_lbl.set_css_classes(&["page-subtitle"]);
            header_row.append(&author_lbl);

            info_box.append(&header_row);

            let desc_lbl = Label::new(Some(&p.description));
            desc_lbl.set_css_classes(&["page-subtitle"]);
            desc_lbl.set_halign(gtk4::Align::Start);
            desc_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
            info_box.append(&desc_lbl);

            row_box.append(&info_box);

            let inst_btn = Button::with_label("📥 Install");
            inst_btn.set_css_classes(&["btn-cyan"]);
            let p_clone = p.clone();
            let cb = on_install.clone();
            inst_btn.connect_clicked(move |_| {
                cb(p_clone.clone());
            });
            row_box.append(&inst_btn);

            row.set_child(Some(&row_box));
            self.search_results_list.append(&row);
        }
    }
}
