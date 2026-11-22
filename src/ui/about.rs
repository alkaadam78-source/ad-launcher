use crate::ui::{create_page_header, create_section_divider};
use crate::utils::system::SystemSpecs;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation};

pub struct AboutView {
    pub container: GtkBox,
}

impl AboutView {
    pub fn new(specs: &SystemSpecs) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 18);
        container.set_css_classes(&["content-area"]);

        // Page Header (Rule 1: Heading with subtle ornamental separator)
        container.append(&create_page_header(
            "CHRONICLES & PEDIGREE OF AD LAUNCHER",
            "The Dark Baroque Edition — High-Performance Native Linux Minecraft Architecture",
        ));

        // Historical Tribute Banner Card
        let creator_card = GtkBox::new(Orientation::Vertical, 8);
        creator_card.set_css_classes(&["banner-card"]);
        let c_title = Label::new(Some("⚜ MASTER CRAFTSMANSHIP & DESIGN: ADAMDEVNOOB ⚜"));
        c_title.set_css_classes(&["banner-title"]);
        c_title.set_halign(gtk4::Align::Start);
        let c_desc = Label::new(Some(
            "Engineered with devotion to high-frame-rate Minecraft gaming on integrated GPUs (Intel HD 620). Featuring lightweight Chiaroscuro shaders, multi-threaded Mesa OpenGL pipelines, and zero-stutter garbage collection."
        ));
        c_desc.set_wrap(true);
        c_desc.set_halign(gtk4::Align::Start);
        creator_card.append(&c_title);
        creator_card.append(&c_desc);
        container.append(&creator_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // Target Hardware Environment Card
        let specs_card = GtkBox::new(Orientation::Vertical, 12);
        specs_card.set_css_classes(&["card"]);

        let s_title = Label::new(Some("CALIBRATED HARDWARE ENVIRONMENT"));
        s_title.set_css_classes(&["card-title"]);
        s_title.set_halign(gtk4::Align::Start);
        specs_card.append(&s_title);

        let sys_info = format!(
            "• Operating System: {} ({})\n\
             • CPU: {} ({} logical execution cores)\n\
             • GPU: Intel HD Graphics 620 (Mesa / Vulkan / OpenGL 4.6 with glthread=true)\n\
             • Total System RAM: {} MB (~8 GB)\n\
             • Available RAM: {} MB\n\
             • Primary Display Target: 1366x768 (Optimized Native GTK4)",
            specs.os_name,
            specs.os_version,
            specs.cpu_name,
            specs.cpu_cores,
            specs.total_memory_mb,
            specs.available_memory_mb
        );

        let specs_lbl = Label::new(Some(&sys_info));
        specs_lbl.set_halign(gtk4::Align::Start);
        specs_lbl.set_css_classes(&["page-subtitle"]);
        specs_card.append(&specs_lbl);

        container.append(&specs_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // Technology Stack Card
        let tech_card = GtkBox::new(Orientation::Vertical, 12);
        tech_card.set_css_classes(&["card"]);

        let t_title = Label::new(Some("TECHNOLOGY STACK & ARCHITECTURAL FOUNDATION"));
        t_title.set_css_classes(&["card-title"]);
        t_title.set_halign(gtk4::Align::Start);
        tech_card.append(&t_title);

        let tech_info = "• Language: Rust 2021 Edition (Memory safety, fearless concurrency, zero-cost abstractions)\n\
                         • GUI Toolkit: GTK4 + Relm4 (Native Wayland/X11 rendering with hardware acceleration)\n\
                         • Asynchronous Engine: Tokio 1.40 (Non-blocking I/O, multi-threaded task spawning)\n\
                         • Network Pipeline: Reqwest + Rustls (Modern TLS, high throughput chunked streaming)\n\
                         • Storage Engine: Embedded SQLite (rusqlite bundled, zero daemon dependencies)\n\
                         • Minecraft Target: 1.20.1 & Modern 1.21.4+ (Fabric + Sodium + Iris + Lithium)\n\
                         • Zero Chromium / Zero Electron / Minimal Idle Memory Footprint";

        let tech_lbl = Label::new(Some(tech_info));
        tech_lbl.set_halign(gtk4::Align::Start);
        tech_lbl.set_css_classes(&["page-subtitle"]);
        tech_card.append(&tech_lbl);

        container.append(&tech_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // License & Heritage Card
        let lic_card = GtkBox::new(Orientation::Vertical, 12);
        lic_card.set_css_classes(&["card"]);

        let l_title = Label::new(Some("HERITAGE & SOFTWARE INTEGRITY"));
        l_title.set_css_classes(&["card-title"]);
        l_title.set_halign(gtk4::Align::Start);
        lic_card.append(&l_title);

        let lic_info = "NUX Launcher Linux / AD Launcher is licensed under the GNU General Public License v3.0.\n\
                        Integrates with official Mojang manifest endpoints and SHA-1 checksum verification.\n\
                        Fully adheres to official Microsoft authentication protocols.";

        let lic_lbl = Label::new(Some(lic_info));
        lic_lbl.set_halign(gtk4::Align::Start);
        lic_lbl.set_css_classes(&["page-subtitle"]);
        lic_card.append(&lic_lbl);

        container.append(&lic_card);

        Self { container }
    }
}
