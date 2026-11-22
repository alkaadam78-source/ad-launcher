use crate::core::shaders::ShaderPackInfo;
use crate::ui::components::{create_empty_state, create_page_header, create_section_divider};
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, CheckButton, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow,
};
use std::rc::Rc;

pub struct ShaderCenterView {
    pub container: GtkBox,
    pub shader_status_label: Label,
    pub toggle_shader_check: CheckButton,
    pub shader_list_box: ListBox,
    pub refresh_btn: Button,
    pub open_folder_btn: Button,
}

impl ShaderCenterView {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 16);
        container.set_css_classes(&["content-area"]);

        // Page Header with Subtle Ornamental Separator (Rule 1)
        let header = create_page_header(
            "THE ILLUMINATION & SHADER GALLERY",
            "Chiaroscuro lighting, shadow tuning, and Iris shaderpack curation",
        );
        container.append(&header);

        // Active Shader Status Card
        let status_card = GtkBox::new(Orientation::Vertical, 12);
        status_card.set_css_classes(&["card"]);

        let status_header = GtkBox::new(Orientation::Horizontal, 12);
        let status_title = Label::new(Some("ACTIVE SHADER ILLUMINATION ENGINE"));
        status_title.set_css_classes(&["card-title"]);
        status_title.set_halign(gtk4::Align::Start);
        status_title.set_hexpand(true);
        status_header.append(&status_title);

        let toggle_shader_check = CheckButton::with_label("Enable Shaders in Game");
        toggle_shader_check.set_active(true);
        status_header.append(&toggle_shader_check);
        status_card.append(&status_header);

        let shader_status_label = Label::new(Some("Active Shader: Checking..."));
        shader_status_label.set_css_classes(&["badge-turbo"]);
        shader_status_label.set_halign(gtk4::Align::Start);
        status_card.append(&shader_status_label);

        let quick_note = Label::new(Some(
            "Curator Note: Miniature and MakeUp UltraFast are pre-configured with 8-chunk shadow distance for maximum Chiaroscuro elegance and fluid FPS on Intel HD 620."
        ));
        quick_note.set_css_classes(&["page-subtitle"]);
        quick_note.set_wrap(true);
        quick_note.set_halign(gtk4::Align::Start);
        status_card.append(&quick_note);

        container.append(&status_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // Installed Shaderpacks List Card
        let list_card = GtkBox::new(Orientation::Vertical, 12);
        list_card.set_css_classes(&["card"]);
        list_card.set_vexpand(true);

        let list_header = GtkBox::new(Orientation::Horizontal, 12);
        let list_title = Label::new(Some(
            "CURATED SHADERPACK ARCHIVE (~/.minecraft/shaderpacks/)",
        ));
        list_title.set_css_classes(&["card-title"]);
        list_title.set_halign(gtk4::Align::Start);
        list_title.set_hexpand(true);
        list_header.append(&list_title);

        let refresh_btn = Button::with_label("↻ Refresh Gallery");
        refresh_btn.set_css_classes(&["btn-amber"]);
        list_header.append(&refresh_btn);

        let open_folder_btn = Button::with_label("📂 Open Shader Vault");
        open_folder_btn.set_css_classes(&["btn-cyan"]);
        list_header.append(&open_folder_btn);

        list_card.append(&list_header);

        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);

        let shader_list_box = ListBox::new();
        shader_list_box.set_selection_mode(gtk4::SelectionMode::None);
        scrolled.set_child(Some(&shader_list_box));
        list_card.append(&scrolled);

        container.append(&list_card);

        Self {
            container,
            shader_status_label,
            toggle_shader_check,
            shader_list_box,
            refresh_btn,
            open_folder_btn,
        }
    }

    pub fn populate_shaders(&self, shaders: &[ShaderPackInfo], on_select: Rc<dyn Fn(String)>) {
        while let Some(child) = self.shader_list_box.first_child() {
            self.shader_list_box.remove(&child);
        }

        if shaders.is_empty() {
            let empty_row = ListBoxRow::new();
            let empty_state = create_empty_state(
                "🕯",
                "Shader Vault Currently Empty",
                "Open the Shader Vault above to add .zip shaderpacks to your collection.",
            );
            empty_row.set_child(Some(&empty_state));
            self.shader_list_box.append(&empty_row);
            return;
        }

        for s in shaders {
            let row = ListBoxRow::new();
            let row_box = GtkBox::new(Orientation::Horizontal, 14);
            row_box.set_css_classes(&["card"]);
            row_box.set_margin_bottom(6);

            let icon_lbl = Label::new(Some("🕯"));
            icon_lbl.set_css_classes(&["page-title"]);
            row_box.append(&icon_lbl);

            let col = GtkBox::new(Orientation::Vertical, 3);
            col.set_hexpand(true);

            let name_row = GtkBox::new(Orientation::Horizontal, 8);
            let name_lbl = Label::new(Some(&s.file_name));
            name_lbl.set_css_classes(&["card-title"]);
            name_lbl.set_halign(gtk4::Align::Start);
            name_row.append(&name_lbl);

            if s.is_active {
                let active_badge = Label::new(Some("ACTIVE SHADER"));
                active_badge.set_css_classes(&["badge-ready"]);
                name_row.append(&active_badge);
            }

            let rating_badge = Label::new(Some(s.compatibility_rating));
            rating_badge.set_css_classes(&["badge-turbo"]);
            name_row.append(&rating_badge);
            col.append(&name_row);

            let sub_info = format!(
                "Size: {} KB • Estimated Intel HD 620 Target: {}",
                s.size_kb, s.estimated_fps
            );
            let sub_lbl = Label::new(Some(&sub_info));
            sub_lbl.set_css_classes(&["page-subtitle"]);
            sub_lbl.set_halign(gtk4::Align::Start);
            col.append(&sub_lbl);

            let rec_lbl = Label::new(Some(s.recommendation));
            rec_lbl.set_wrap(true);
            rec_lbl.set_halign(gtk4::Align::Start);
            rec_lbl.set_css_classes(&["page-subtitle"]);
            col.append(&rec_lbl);

            row_box.append(&col);

            let sel_btn = Button::with_label(if s.is_active {
                "✔ Activated"
            } else {
                "⚔ Activate Shader"
            });
            sel_btn.set_css_classes(&[if s.is_active {
                "btn-emerald"
            } else {
                "btn-cyan"
            }]);
            sel_btn.set_valign(gtk4::Align::Center);

            let fname = s.file_name.clone();
            let cb = on_select.clone();
            sel_btn.connect_clicked(move |_| {
                cb(fname.clone());
            });
            row_box.append(&sel_btn);

            row.set_child(Some(&row_box));
            self.shader_list_box.append(&row);
        }
    }
}
