use crate::core::fps_booster::FpsOptimizationMode;
use crate::ui::components::{create_page_header, create_section_divider};
use crate::utils::system::SystemSpecs;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Label, Orientation};

pub struct PerformanceCenterView {
    pub container: GtkBox,
    pub apply_hyper_potato_btn: Button,
    pub apply_shader_smooth_btn: Button,
    pub apply_balanced_btn: Button,
    pub apply_battery_saver_btn: Button,
    pub feedback_label: Label,
}

impl PerformanceCenterView {
    pub fn new(specs: &SystemSpecs) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 16);
        container.set_css_classes(&["content-area"]);

        // Page Header with Subtle Ornamental Separator (Rule 1)
        let header = create_page_header(
            "ROYAL PERFORMANCE LAB & TELEMETRY",
            "Real-time hardware telemetry and classical optimization treatises",
        );
        container.append(&header);

        // Feedback / Notification Banner
        let feedback_label = Label::new(Some(
            "Optimization Lab: Select an optimization treatise below to apply configuration.",
        ));
        feedback_label.set_css_classes(&["banner-card"]);
        feedback_label.set_halign(gtk4::Align::Fill);
        container.append(&feedback_label);

        // Scientific Instrument Telemetry Card (Rule 9: Hierarchy accent)
        let telemetry_card = GtkBox::new(Orientation::Vertical, 10);
        telemetry_card.set_css_classes(&["card"]);

        let t_title = Label::new(Some("PHILOSOPHICAL INSTRUMENT TELEMETRY & HARDWARE TARGET"));
        t_title.set_css_classes(&["card-title"]);
        t_title.set_halign(gtk4::Align::Start);
        telemetry_card.append(&t_title);

        let row1 = GtkBox::new(Orientation::Horizontal, 12);
        let cpu_badge = Label::new(Some(&format!(
            "CPU: {} ({} Cores)",
            specs.cpu_name, specs.cpu_cores
        )));
        cpu_badge.set_css_classes(&["badge-info"]);
        let gpu_badge = Label::new(Some("GPU: Intel HD Graphics 620 (Mesa glthread=ON)"));
        gpu_badge.set_css_classes(&["badge-turbo"]);
        let ram_badge = Label::new(Some(&format!(
            "RAM: {} MB / Safe Max: {} MB",
            specs.total_memory_mb, specs.max_safe_ram_mb
        )));
        ram_badge.set_css_classes(&["badge-ready"]);

        row1.append(&cpu_badge);
        row1.append(&gpu_badge);
        row1.append(&ram_badge);
        telemetry_card.append(&row1);

        let t_desc = Label::new(Some(
            "Mesa multi-threaded OpenGL command queue (`mesa_glthread=true`) is active. Eliminates driver bottlenecks and maximizes frame throughput on integrated graphics."
        ));
        t_desc.set_css_classes(&["page-subtitle"]);
        t_desc.set_wrap(true);
        t_desc.set_halign(gtk4::Align::Start);
        telemetry_card.append(&t_desc);

        container.append(&telemetry_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // Performance Optimization Modes
        let modes_title = Label::new(Some("OPTIMIZATION TREATISES (SELECT PERFORMANCE REGIME)"));
        modes_title.set_css_classes(&["card-title"]);
        modes_title.set_halign(gtk4::Align::Start);
        container.append(&modes_title);

        let grid_row1 = GtkBox::new(Orientation::Horizontal, 12);
        let grid_row2 = GtkBox::new(Orientation::Horizontal, 12);

        // Mode 1: Adam Hyper Potato
        let (card1, apply_hyper_potato_btn) = Self::create_mode_card(
            FpsOptimizationMode::AdamHyperPotato,
            "⚡ 200+ FPS (POTATO ALCHEMY)",
            "btn-play",
        );
        card1.set_hexpand(true);
        grid_row1.append(&card1);

        // Mode 2: Adam Shader Smooth
        let (card2, apply_shader_smooth_btn) = Self::create_mode_card(
            FpsOptimizationMode::AdamShaderSmooth,
            "✨ 100-140 FPS (CHIAROSCURO)",
            "btn-cyan",
        );
        card2.set_hexpand(true);
        grid_row1.append(&card2);

        // Mode 3: Balanced Edutainment
        let (card3, apply_balanced_btn) = Self::create_mode_card(
            FpsOptimizationMode::BalancedEdutainment,
            "🎮 80-120 FPS (HARMONIOUS)",
            "btn-purple",
        );
        card3.set_hexpand(true);
        grid_row2.append(&card3);

        // Mode 4: Battery Saver
        let (card4, apply_battery_saver_btn) = Self::create_mode_card(
            FpsOptimizationMode::BatterySaver,
            "🔋 60 FPS (NOCTURNE SAVER)",
            "btn-emerald",
        );
        card4.set_hexpand(true);
        grid_row2.append(&card4);

        container.append(&grid_row1);
        container.append(&grid_row2);

        Self {
            container,
            apply_hyper_potato_btn,
            apply_shader_smooth_btn,
            apply_balanced_btn,
            apply_battery_saver_btn,
            feedback_label,
        }
    }

    fn create_mode_card(
        mode: FpsOptimizationMode,
        badge_text: &str,
        btn_class: &str,
    ) -> (GtkBox, Button) {
        let card = GtkBox::new(Orientation::Vertical, 8);
        card.set_css_classes(&["card"]);

        let header = GtkBox::new(Orientation::Horizontal, 8);
        let title_lbl = Label::new(Some(mode.title()));
        title_lbl.set_css_classes(&["card-title"]);
        title_lbl.set_halign(gtk4::Align::Start);
        title_lbl.set_hexpand(true);
        header.append(&title_lbl);

        let badge = Label::new(Some(badge_text));
        badge.set_css_classes(&["badge-turbo"]);
        header.append(&badge);
        card.append(&header);

        let desc_lbl = Label::new(Some(mode.description()));
        desc_lbl.set_css_classes(&["page-subtitle"]);
        desc_lbl.set_wrap(true);
        desc_lbl.set_halign(gtk4::Align::Start);
        card.append(&desc_lbl);

        let specs_info = format!(
            "RAM: {} MB • Render: {} Chunks • Simulation: {} Chunks • Target: {} FPS",
            mode.ram_mb(),
            mode.render_distance(),
            mode.simulation_distance(),
            mode.max_fps()
        );
        let specs_lbl = Label::new(Some(&specs_info));
        specs_lbl.set_css_classes(&["badge-info"]);
        specs_lbl.set_halign(gtk4::Align::Start);
        card.append(&specs_lbl);

        let apply_btn = Button::with_label("⚔ Apply This Treatise");
        apply_btn.set_css_classes(&[btn_class]);
        apply_btn.set_margin_top(6);
        card.append(&apply_btn);

        (card, apply_btn)
    }
}
