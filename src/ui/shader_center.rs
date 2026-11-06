use crate::core::shaders::ShaderPackInfo;
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

        // Page Header
        let header_row = GtkBox::new(Orientation::Horizontal, 12);
        let title_col = GtkBox::new(Orientation::Vertical, 2);

        let title = Label::new(Some("✨ MOD & SHADER CENTER"));
        title.set_css_classes(&["page-title"]);
        title.set_halign(gtk4::Align::Start);

        let subtitle = Label::new(Some("Integrated Iris Shaderpack & Mod Management (Potato Tuned)"));
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

        // Active Shader Status Card
        let status_card = GtkBox::new(Orientation::Vertical, 12);
        status_card.set_css_classes(&["card"]);

        let status_header = GtkBox::new(Orientation::Horizontal, 12);
        let status_title = Label::new(Some("CURRENT ACTIVE SHADER ENGINE"));
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
            "💡 Pro-Tip: Miniature and MakeUp UltraFast are pre-configured with 8-chunk shadow distance for max FPS on Intel HD 620."
        ));
        quick_note.set_css_classes(&["page-subtitle"]);
        quick_note.set_wrap(true);
        quick_note.set_halign(gtk4::Align::Start);
        status_card.append(&quick_note);

        container.append(&status_card);

        // Installed Shaderpacks List Card
        let list_card = GtkBox::new(Orientation::Vertical, 12);
        list_card.set_css_classes(&["card"]);
        list_card.set_vexpand(true);

        let list_header = GtkBox::new(Orientation::Horizontal, 12);
        let list_title = Label::new(Some("AVAILABLE SHADERPACKS IN ~/.minecraft/shaderpacks/"));
        list_title.set_css_classes(&["card-title"]);
        list_title.set_halign(gtk4::Align::Start);
        list_title.set_hexpand(true);
        list_header.append(&list_title);

        let refresh_btn = Button::with_label("↻ Refresh Shaders");
        refresh_btn.set_css_classes(&["btn-secondary"]);
        list_header.append(&refresh_btn);

        let open_folder_btn = Button::with_label("📂 Open Shaderpacks Folder");
        open_folder_btn.set_css_classes(&["btn-secondary"]);
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

    pub fn populate_shaders(
        &self,
        shaders: &[ShaderPackInfo],
        on_select: Rc<dyn Fn(String)>,
    ) {
        while let Some(child) = self.shader_list_box.first_child() {
            self.shader_list_box.remove(&child);
        }

        if shaders.is_empty() {
            let empty_lbl = Label::new(Some("No shaderpacks (.zip) found in ~/.minecraft/shaderpacks/"));
            empty_lbl.set_margin_top(16);
            empty_lbl.set_css_classes(&["page-subtitle"]);
            self.shader_list_box.append(&empty_lbl);
            return;
        }

        for s in shaders {
            let row = ListBoxRow::new();
            let row_box = GtkBox::new(Orientation::Horizontal, 12);
            row_box.set_margin_top(6);
            row_box.set_margin_bottom(6);
            row_box.set_margin_start(8);
            row_box.set_margin_end(8);

            let info_col = GtkBox::new(Orientation::Vertical, 2);
            info_col.set_hexpand(true);

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

            info_col.append(&name_row);

            let sub_info = format!("Size: {} KB • Estimated Intel HD 620 FPS: {}", s.size_kb, s.estimated_fps);
            let sub_lbl = Label::new(Some(&sub_info));
            sub_lbl.set_css_classes(&["page-subtitle"]);
            sub_lbl.set_halign(gtk4::Align::Start);
            info_col.append(&sub_lbl);

            let rec_lbl = Label::new(Some(s.recommendation));
            rec_lbl.set_wrap(true);
            rec_lbl.set_halign(gtk4::Align::Start);
            rec_lbl.set_css_classes(&["page-subtitle"]);
            info_col.append(&rec_lbl);

            row_box.append(&info_col);

            let select_btn = Button::with_label(if s.is_active { "✔ Activated" } else { "Activate Shader" });
            select_btn.set_css_classes(&[if s.is_active { "btn-primary" } else { "btn-secondary" }]);
            select_btn.set_valign(gtk4::Align::Center);

            let name_clone = s.file_name.clone();
            let on_sel = on_select.clone();
            select_btn.connect_clicked(move |_| {
                on_sel(name_clone.clone());
            });

            row_box.append(&select_btn);

            row.set_child(Some(&row_box));
            self.shader_list_box.append(&row);
        }
    }
}
