use crate::core::instances::Instance;
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, Entry, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow,
};
use std::rc::Rc;

pub struct InstancesView {
    pub container: GtkBox,
    pub list_box: ListBox,
    pub name_entry: Entry,
    pub create_btn: Button,
    pub refresh_btn: Button,
}

impl InstancesView {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 16);
        container.set_css_classes(&["content-area"]);

        // Page Header
        let header_row = GtkBox::new(Orientation::Horizontal, 12);
        let title_col = GtkBox::new(Orientation::Vertical, 2);

        let title = Label::new(Some("📚 MINECRAFT LIBRARY"));
        title.set_css_classes(&["page-title"]);
        title.set_halign(gtk4::Align::Start);

        let subtitle = Label::new(Some("Game Versions, Fabric Loaders, Custom Instances & Fast Launcher"));
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

        // Instance creation card
        let form_card = GtkBox::new(Orientation::Vertical, 10);
        form_card.set_css_classes(&["card"]);

        let form_title = Label::new(Some("CREATE NEW INSTANCE"));
        form_title.set_css_classes(&["card-title"]);
        form_title.set_halign(gtk4::Align::Start);
        form_card.append(&form_title);

        let row_box = GtkBox::new(Orientation::Horizontal, 12);
        let name_entry = Entry::new();
        name_entry.set_placeholder_text(Some("Instance Name (e.g. My Survival 1.20.1)"));
        name_entry.set_hexpand(true);
        row_box.append(&name_entry);

        let create_btn = Button::with_label("+ Create Instance");
        create_btn.set_css_classes(&["btn-primary"]);
        row_box.append(&create_btn);

        form_card.append(&row_box);
        container.append(&form_card);

        // Instance List Card
        let list_card = GtkBox::new(Orientation::Vertical, 10);
        list_card.set_css_classes(&["card"]);
        list_card.set_vexpand(true);

        let header_row = GtkBox::new(Orientation::Horizontal, 12);
        let list_title = Label::new(Some("AVAILABLE INSTANCES"));
        list_title.set_css_classes(&["card-title"]);
        list_title.set_hexpand(true);
        list_title.set_halign(gtk4::Align::Start);
        header_row.append(&list_title);

        let refresh_btn = Button::with_label("↻ Refresh");
        refresh_btn.set_css_classes(&["btn-secondary"]);
        header_row.append(&refresh_btn);

        list_card.append(&header_row);

        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);

        let list_box = ListBox::new();
        list_box.set_selection_mode(gtk4::SelectionMode::None);
        scrolled.set_child(Some(&list_box));

        list_card.append(&scrolled);
        container.append(&list_card);

        Self {
            container,
            list_box,
            name_entry,
            create_btn,
            refresh_btn,
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
        // Clear existing rows
        while let Some(child) = self.list_box.first_child() {
            self.list_box.remove(&child);
        }

        for inst in instances {
            let row = ListBoxRow::new();
            let row_box = GtkBox::new(Orientation::Horizontal, 12);
            row_box.set_margin_top(8);
            row_box.set_margin_bottom(8);
            row_box.set_margin_start(12);
            row_box.set_margin_end(12);

            let is_active = inst.id == active_id;

            let info_box = GtkBox::new(Orientation::Vertical, 4);
            info_box.set_hexpand(true);

            let title_box = GtkBox::new(Orientation::Horizontal, 8);
            let name_lbl = Label::new(Some(&inst.name));
            name_lbl.set_css_classes(&["card-title"]);
            name_lbl.set_halign(gtk4::Align::Start);
            title_box.append(&name_lbl);

            if is_active {
                let badge = Label::new(Some("ACTIVE"));
                badge.set_css_classes(&["badge-ready"]);
                title_box.append(&badge);
            }

            info_box.append(&title_box);

            let desc = format!(
                "Minecraft {} • Loader: {} • RAM: {} MB • Path: {}",
                inst.mc_version,
                inst.loader,
                inst.ram_mb,
                inst.game_dir.display()
            );
            let desc_lbl = Label::new(Some(&desc));
            desc_lbl.set_halign(gtk4::Align::Start);
            desc_lbl.set_css_classes(&["page-subtitle"]);
            info_box.append(&desc_lbl);

            row_box.append(&info_box);

            // Action Buttons
            if !is_active {
                let sel_btn = Button::with_label("Select");
                sel_btn.set_css_classes(&["btn-secondary"]);
                let id_clone = inst.id.clone();
                let sel_cb = on_select.clone();
                sel_btn.connect_clicked(move |_| {
                    sel_cb(id_clone.clone());
                });
                row_box.append(&sel_btn);
            }

            let folder_btn = Button::with_label("📁 Folder");
            folder_btn.set_css_classes(&["btn-secondary"]);
            let dir_clone = inst.game_dir.clone();
            let folder_cb = on_open_folder.clone();
            folder_btn.connect_clicked(move |_| {
                folder_cb(dir_clone.clone());
            });
            row_box.append(&folder_btn);

            if inst.id != "default-1.20.1-fabric" {
                let del_btn = Button::with_label("Delete");
                del_btn.set_css_classes(&["btn-danger"]);
                let id_clone = inst.id.clone();
                let del_cb = on_delete.clone();
                del_btn.connect_clicked(move |_| {
                    del_cb(id_clone.clone());
                });
                row_box.append(&del_btn);
            }

            row.set_child(Some(&row_box));
            self.list_box.append(&row);
        }
    }
}
