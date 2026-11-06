use crate::core::shaders::{ShaderPackInfo, ShaderTier};
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, Label, ListBox, ListBoxRow, Orientation, ProgressBar, ScrolledWindow,
};

pub struct CompatibilityLabView {
    pub container: GtkBox,
    pub run_benchmark_btn: Button,
    pub apply_potato_calibration_btn: Button,
    pub progress_bar: ProgressBar,
    pub lab_status_label: Label,
    pub results_list: ListBox,
}

impl CompatibilityLabView {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 16);
        container.set_css_classes(&["content-area"]);

        // Page Header
        let header_row = GtkBox::new(Orientation::Horizontal, 12);
        let title_col = GtkBox::new(Orientation::Vertical, 2);

        let title = Label::new(Some("🧪 SHADERS COMPATIBILITY LAB"));
        title.set_css_classes(&["page-title"]);
        title.set_halign(gtk4::Align::Start);

        let subtitle = Label::new(Some("Intel HD Graphics 620 Vulkan/OpenGL Shader Diagnostics & Potato Calibrator"));
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

        // Lab Controls Card
        let control_card = GtkBox::new(Orientation::Vertical, 10);
        control_card.set_css_classes(&["card"]);

        let c_title = Label::new(Some("INTEL HD 620 SHADER BENCHMARK & AUTO-CALIBRATION"));
        c_title.set_css_classes(&["card-title"]);
        c_title.set_halign(gtk4::Align::Start);
        control_card.append(&c_title);

        let c_desc = Label::new(Some(
            "Analyzes GLSL complexity, shadow map resolution, and post-processing shaders against Intel HD 620 execution units. Auto-tunes iris.properties to avoid GPU throttling."
        ));
        c_desc.set_css_classes(&["page-subtitle"]);
        c_desc.set_wrap(true);
        c_desc.set_halign(gtk4::Align::Start);
        control_card.append(&c_desc);

        let btn_row = GtkBox::new(Orientation::Horizontal, 12);
        let run_benchmark_btn = Button::with_label("🧪 Run Shader Compatibility Scan");
        run_benchmark_btn.set_css_classes(&["btn-primary"]);

        let apply_potato_calibration_btn = Button::with_label("⚡ Apply Potato Calibration (Shadows: 8 Chunks, No DOF)");
        apply_potato_calibration_btn.set_css_classes(&["btn-secondary"]);

        btn_row.append(&run_benchmark_btn);
        btn_row.append(&apply_potato_calibration_btn);
        control_card.append(&btn_row);

        let progress_bar = ProgressBar::new();
        progress_bar.set_fraction(1.0);
        progress_bar.set_visible(false);
        control_card.append(&progress_bar);

        let lab_status_label = Label::new(Some("System Ready. 5 Shaders detected in laboratory."));
        lab_status_label.set_css_classes(&["badge-ready"]);
        lab_status_label.set_halign(gtk4::Align::Start);
        control_card.append(&lab_status_label);

        container.append(&control_card);

        // Detailed Shader Lab Results Card
        let results_card = GtkBox::new(Orientation::Vertical, 10);
        results_card.set_css_classes(&["card"]);
        results_card.set_vexpand(true);

        let r_title = Label::new(Some("SHADER LAB COMPATIBILITY MATRIX (INTEL HD 620 / LINUX MESA)"));
        r_title.set_css_classes(&["card-title"]);
        r_title.set_halign(gtk4::Align::Start);
        results_card.append(&r_title);

        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);

        let results_list = ListBox::new();
        results_list.set_selection_mode(gtk4::SelectionMode::None);
        scrolled.set_child(Some(&results_list));
        results_card.append(&scrolled);

        container.append(&results_card);

        Self {
            container,
            run_benchmark_btn,
            apply_potato_calibration_btn,
            progress_bar,
            lab_status_label,
            results_list,
        }
    }

    pub fn populate_lab_results(&self, shaders: &[ShaderPackInfo]) {
        while let Some(child) = self.results_list.first_child() {
            self.results_list.remove(&child);
        }

        for s in shaders {
            let row = ListBoxRow::new();
            let row_box = GtkBox::new(Orientation::Horizontal, 12);
            row_box.set_margin_top(6);
            row_box.set_margin_bottom(6);
            row_box.set_margin_start(8);
            row_box.set_margin_end(8);

            let icon_lbl = Label::new(Some(match s.tier {
                ShaderTier::PotatoFriendly => "🥔",
                ShaderTier::UltraFast => "⚡",
                ShaderTier::Balanced => "✨",
                ShaderTier::Heavy => "⚠️",
            }));
            icon_lbl.set_css_classes(&["page-title"]);
            icon_lbl.set_valign(gtk4::Align::Center);
            row_box.append(&icon_lbl);

            let info_col = GtkBox::new(Orientation::Vertical, 2);
            info_col.set_hexpand(true);

            let title_row = GtkBox::new(Orientation::Horizontal, 8);
            let name_lbl = Label::new(Some(&s.file_name));
            name_lbl.set_css_classes(&["card-title"]);
            name_lbl.set_halign(gtk4::Align::Start);
            title_row.append(&name_lbl);

            let comp_badge = Label::new(Some(s.compatibility_rating));
            comp_badge.set_css_classes(&[match s.tier {
                ShaderTier::PotatoFriendly => "badge-ready",
                ShaderTier::UltraFast => "badge-turbo",
                ShaderTier::Balanced => "badge-info",
                ShaderTier::Heavy => "badge-warning",
            }]);
            title_row.append(&comp_badge);

            info_col.append(&title_row);

            let lab_analysis = format!(
                "Lab Prediction: {} • Status: {}",
                s.estimated_fps,
                s.recommendation
            );
            let lab_lbl = Label::new(Some(&lab_analysis));
            lab_lbl.set_css_classes(&["page-subtitle"]);
            lab_lbl.set_wrap(true);
            lab_lbl.set_halign(gtk4::Align::Start);
            info_col.append(&lab_lbl);

            row_box.append(&info_col);
            row.set_child(Some(&row_box));
            self.results_list.append(&row);
        }
    }
}
