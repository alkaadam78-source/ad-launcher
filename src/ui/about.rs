use crate::utils::system::SystemSpecs;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation};

pub struct AboutView {
    pub container: GtkBox,
}

impl AboutView {
    pub fn new(specs: &SystemSpecs) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 16);
        container.set_css_classes(&["content-area"]);

        let title = Label::new(Some("ABOUT NUX LAUNCHER LINUX"));
        title.set_css_classes(&["page-title"]);
        title.set_halign(gtk4::Align::Start);

        let subtitle = Label::new(Some("Personal Edition — Native Rust Desktop Launcher for Debian"));
        subtitle.set_css_classes(&["page-subtitle"]);
        subtitle.set_halign(gtk4::Align::Start);

        container.append(&title);
        container.append(&subtitle);

        // System Specs Card
        let specs_card = GtkBox::new(Orientation::Vertical, 10);
        specs_card.set_css_classes(&["card"]);

        let s_title = Label::new(Some("TARGET HARDWARE ENVIRONMENT"));
        s_title.set_css_classes(&["card-title"]);
        s_title.set_halign(gtk4::Align::Start);
        specs_card.append(&s_title);

        let sys_info = format!(
            "• Operating System: {} ({})\n\
             • CPU: {} ({} logical cores)\n\
             • GPU: Intel HD Graphics 620 (Mesa / Vulkan / OpenGL 4.6)\n\
             • Total System RAM: {} MB (~8 GB)\n\
             • Available RAM: {} MB\n\
             • Default Target Resolution: 1366x768",
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

        // Tech Stack Card
        let tech_card = GtkBox::new(Orientation::Vertical, 10);
        tech_card.set_css_classes(&["card"]);

        let t_title = Label::new(Some("TECHNOLOGY STACK & ARCHITECTURE"));
        t_title.set_css_classes(&["card-title"]);
        t_title.set_halign(gtk4::Align::Start);
        tech_card.append(&t_title);

        let tech_info = "• Language: Rust 2021 (Zero-cost abstractions, memory safe)\n\
                         • GUI Framework: GTK4 + Relm4 (Native Linux graphics integration)\n\
                         • Async Runtime: Tokio 1.40 (Non-blocking I/O & downloads)\n\
                         • HTTP Engine: Reqwest with Rustls (Modern TLS, high throughput)\n\
                         • Storage: Embedded SQLite (rusqlite bundled)\n\
                         • Target Version: Minecraft Java Edition 1.20.1 (Fabric + Sodium + Iris)\n\
                         • Zero Chromium / Zero Electron background bloat";

        let tech_lbl = Label::new(Some(tech_info));
        tech_lbl.set_halign(gtk4::Align::Start);
        tech_lbl.set_css_classes(&["page-subtitle"]);
        tech_card.append(&tech_lbl);

        container.append(&tech_card);

        // License & Credits Card
        let lic_card = GtkBox::new(Orientation::Vertical, 10);
        lic_card.set_css_classes(&["card"]);

        let l_title = Label::new(Some("LICENSE & INTEGRITY"));
        l_title.set_css_classes(&["card-title"]);
        l_title.set_halign(gtk4::Align::Start);
        lic_card.append(&l_title);

        let lic_info = "NUX Launcher Linux is released under the GNU General Public License v3.0.\n\
                        Uses official Mojang download endpoints and checksum verification.\n\
                        Respects Microsoft authentication and Minecraft ownership requirements.";

        let lic_lbl = Label::new(Some(lic_info));
        lic_lbl.set_halign(gtk4::Align::Start);
        lic_lbl.set_css_classes(&["page-subtitle"]);
        lic_card.append(&lic_lbl);

        container.append(&lic_card);

        Self { container }
    }
}
