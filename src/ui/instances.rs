use crate::core::instances::Instance;
use crate::ui::components::{create_empty_state, create_page_header, create_section_divider};
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Entry, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow};
use std::rc::Rc;

pub struct InstancesView {
    pub container: GtkBox,
    pub list_box: ListBox,
    pub name_entry: Entry,
    pub version_entry: Entry,
    pub create_btn: Button,
    pub refresh_btn: Button,
    pub import_btn: Button,
    pub export_btn: Button,
    pub snapshot_btn: Button,
}

impl InstancesView {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 16);
        container.set_css_classes(&["content-area"]);

        // Page Header with Ornamental Separator (Rule 1)
        let header = create_page_header(
            "COLLECTION OF WORLDS & INSTANCES",
            "Curated Minecraft version archives, custom instances & loader profiles",
        );
        container.append(&header);

        // Commission Instance Profile Card
        let form_card = GtkBox::new(Orientation::Vertical, 12);
        form_card.set_css_classes(&["card"]);

        let form_title = Label::new(Some("COMMISSION NEW INSTANCE PROFILE"));
        form_title.set_css_classes(&["card-title"]);
        form_title.set_halign(gtk4::Align::Start);
        form_card.append(&form_title);

        let row_box = GtkBox::new(Orientation::Horizontal, 12);

        let name_entry = Entry::new();
        name_entry.set_placeholder_text(Some("Instance Title (e.g. Royal Survival 1.20.1)"));
        name_entry.set_hexpand(true);
        row_box.append(&name_entry);

        let version_entry = Entry::new();
        version_entry.set_placeholder_text(Some("Version (1.20.1, 1.21.4, etc.)"));
        version_entry.set_text("1.20.1");
        version_entry.set_width_chars(16);
        row_box.append(&version_entry);

        let create_btn = Button::with_label("+ Commission Instance");
        create_btn.set_css_classes(&["btn-cyan"]);
        row_box.append(&create_btn);

        form_card.append(&row_box);

        // Quick Version Selector Chips
        let chips_box = GtkBox::new(Orientation::Horizontal, 8);
        chips_box.set_margin_top(4);
        let chips_lbl = Label::new(Some("Select Epoch:"));
        chips_lbl.set_css_classes(&["page-subtitle"]);
        chips_box.append(&chips_lbl);

        for ver in ["1.21.4", "1.20.1", "1.16.5", "1.12.2", "1.8.9", "1.7.10"] {
            let chip_btn = Button::with_label(ver);
            chip_btn.set_css_classes(&["filter-btn"]);
            let ver_clone = ver.to_string();
            let v_ent = version_entry.clone();
            chip_btn.connect_clicked(move |_| {
                v_ent.set_text(&ver_clone);
            });
            chips_box.append(&chip_btn);
        }
        form_card.append(&chips_box);

        // Instance Portability & Snapshots Toolbar (Master Prompt 05)
        let action_row = GtkBox::new(Orientation::Horizontal, 8);
        action_row.set_margin_top(6);

        let import_btn = Button::with_label("🔄 Scan & Import External Instances");
        import_btn.set_css_classes(&["btn-amber"]);

        let export_btn = Button::with_label("📦 Export Instance Archive");
        export_btn.set_css_classes(&["btn-secondary"]);

        let snapshot_btn = Button::with_label("📸 Create Snapshot");
        snapshot_btn.set_css_classes(&["btn-secondary"]);

        action_row.append(&import_btn);
        action_row.append(&export_btn);
        action_row.append(&snapshot_btn);
        form_card.append(&action_row);

        container.append(&form_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // Instance List Card
        let list_card = GtkBox::new(Orientation::Vertical, 12);
        list_card.set_css_classes(&["card"]);
        list_card.set_vexpand(true);

        let header_row = GtkBox::new(Orientation::Horizontal, 12);
        let list_title = Label::new(Some("ARCHIVED INSTANCE CATALOG"));
        list_title.set_css_classes(&["card-title"]);
        list_title.set_hexpand(true);
        list_title.set_halign(gtk4::Align::Start);
        header_row.append(&list_title);

        let refresh_btn = Button::with_label("↻ Refresh Catalog");
        refresh_btn.set_css_classes(&["btn-amber"]);
        header_row.append(&refresh_btn);

        list_card.append(&header_row);

        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);
        scrolled.set_min_content_height(350);

        let list_box = ListBox::new();
        list_box.set_selection_mode(gtk4::SelectionMode::None);
        scrolled.set_child(Some(&list_box));

        list_card.append(&scrolled);
        container.append(&list_card);

        Self {
            container,
            list_box,
            name_entry,
            version_entry,
            create_btn,
            refresh_btn,
            import_btn,
            export_btn,
            snapshot_btn,
        }
    }

    pub fn populate_instances(
        &self,
        instances: &[Instance],
        active_id: &str,
        on_select: Rc<dyn Fn(String)>,
        on_open_folder: Rc<dyn Fn(std::path::PathBuf)>,
        on_delete: Rc<dyn Fn(String)>,
    ) {
        while let Some(child) = self.list_box.first_child() {
            self.list_box.remove(&child);
        }

        if instances.is_empty() {
            let empty_row = ListBoxRow::new();
            let empty_state = create_empty_state(
                "📜",
                "No World Instances Cataloged",
                "Commission your first instance profile above to begin your journey.",
            );
            empty_row.set_child(Some(&empty_state));
            self.list_box.append(&empty_row);
            return;
        }

        for inst in instances {
            let row = ListBoxRow::new();
            let row_box = GtkBox::new(Orientation::Horizontal, 14);
            row_box.set_css_classes(&["card"]);
            row_box.set_margin_bottom(6);

            let is_active = inst.id == active_id;

            let icon_lbl = Label::new(Some(if is_active { "⚜" } else { "📜" }));
            icon_lbl.set_css_classes(&["page-title"]);
            row_box.append(&icon_lbl);

            let info_col = GtkBox::new(Orientation::Vertical, 2);
            info_col.set_hexpand(true);

            let name_row = GtkBox::new(Orientation::Horizontal, 8);
            let name_lbl = Label::new(Some(&inst.name));
            name_lbl.set_css_classes(&["card-title"]);
            name_lbl.set_halign(gtk4::Align::Start);
            name_row.append(&name_lbl);

            if is_active {
                let active_badge = Label::new(Some("• ACTIVE WORLD •"));
                active_badge.set_css_classes(&["badge-ready"]);
                name_row.append(&active_badge);
            }
            info_col.append(&name_row);

            let meta_text = format!(
                "Minecraft {} • Loader: {} {} • RAM: {} MB",
                inst.mc_version,
                inst.loader,
                if inst.loader_version.is_empty() {
                    ""
                } else {
                    &inst.loader_version
                },
                inst.ram_mb
            );
            let meta_lbl = Label::new(Some(&meta_text));
            meta_lbl.set_css_classes(&["page-subtitle"]);
            meta_lbl.set_halign(gtk4::Align::Start);
            info_col.append(&meta_lbl);

            row_box.append(&info_col);

            let btn_box = GtkBox::new(Orientation::Horizontal, 8);

            let folder_btn = Button::with_label("📂 Folder");
            folder_btn.set_css_classes(&["btn-secondary"]);
            let f_path = inst.game_dir.clone();
            let cb_folder = on_open_folder.clone();
            folder_btn.connect_clicked(move |_| {
                cb_folder(f_path.clone());
            });
            btn_box.append(&folder_btn);

            if !is_active {
                let select_btn = Button::with_label("⚔ Select");
                select_btn.set_css_classes(&["btn-cyan"]);
                let id_clone = inst.id.clone();
                let cb_select = on_select.clone();
                select_btn.connect_clicked(move |_| {
                    cb_select(id_clone.clone());
                });
                btn_box.append(&select_btn);

                let del_btn = Button::with_label("✕ Delete");
                del_btn.set_css_classes(&["btn-danger"]);
                let id_del = inst.id.clone();
                let cb_del = on_delete.clone();
                del_btn.connect_clicked(move |_| {
                    cb_del(id_del.clone());
                });
                btn_box.append(&del_btn);
            } else {
                let active_tag = Button::with_label("✓ Active");
                active_tag.set_css_classes(&["btn-emerald"]);
                active_tag.set_sensitive(false);
                btn_box.append(&active_tag);
            }

            row_box.append(&btn_box);
            row.set_child(Some(&row_box));
            self.list_box.append(&row);
        }
    }
}
