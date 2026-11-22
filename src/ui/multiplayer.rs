use crate::core::multiplayer::{MinecraftServerEntry, MultiplayerManager, ServerPingResult};
use crate::ui::components::{create_page_header, create_section_divider};
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Entry, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MultiplayerView {
    pub container: GtkBox,
    pub server_list_box: ListBox,
    pub refresh_btn: Button,
    pub add_server_btn: Button,
    pub server_name_entry: Entry,
    pub server_addr_entry: Entry,
    pub status_label: Label,
    servers: Rc<RefCell<Vec<MinecraftServerEntry>>>,
}

impl MultiplayerView {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 18);
        container.set_css_classes(&["content-area"]);

        // Page Header (Rule 1)
        container.append(&create_page_header(
            "COURT MULTIPLAYER & REALMS",
            "Explore curated classical realms, manage LAN connections, and audit server compatibility",
        ));

        // Add Server Card
        let add_card = GtkBox::new(Orientation::Vertical, 12);
        add_card.set_css_classes(&["card"]);

        let add_title = Label::new(Some("COMMISSION EXTERNAL REALM (ADD SERVER)"));
        add_title.set_css_classes(&["card-title"]);
        add_title.set_halign(gtk4::Align::Start);
        add_card.append(&add_title);

        let input_row = GtkBox::new(Orientation::Horizontal, 10);
        let server_name_entry = Entry::new();
        server_name_entry.set_placeholder_text(Some("Realm Name (e.g. Royal Archives SMP)"));
        server_name_entry.set_hexpand(true);
        input_row.append(&server_name_entry);

        let server_addr_entry = Entry::new();
        server_addr_entry.set_placeholder_text(Some("Server Address:Port (e.g. mc.example.com)"));
        server_addr_entry.set_hexpand(true);
        input_row.append(&server_addr_entry);

        let add_server_btn = Button::with_label("+ Add Realm");
        add_server_btn.set_css_classes(&["btn-play"]);
        input_row.append(&add_server_btn);
        add_card.append(&input_row);

        let status_label = Label::new(Some(
            "Multiplayer Engine: Ready to probe server latency and compatibility.",
        ));
        status_label.set_css_classes(&["badge-ready"]);
        status_label.set_halign(gtk4::Align::Start);
        add_card.append(&status_label);

        container.append(&add_card);

        // Section Divider (Rule 2)
        container.append(&create_section_divider());

        // Curated & Known Realms Card
        let list_card = GtkBox::new(Orientation::Vertical, 12);
        list_card.set_css_classes(&["card"]);
        list_card.set_vexpand(true);

        let list_header = GtkBox::new(Orientation::Horizontal, 12);
        let list_title = Label::new(Some("CURATED CLASSICAL REALMS & DISCOVERED SERVERS"));
        list_title.set_css_classes(&["card-title"]);
        list_title.set_halign(gtk4::Align::Start);
        list_title.set_hexpand(true);
        list_header.append(&list_title);

        let refresh_btn = Button::with_label("↻ Ping All Realms");
        refresh_btn.set_css_classes(&["btn-cyan"]);
        list_header.append(&refresh_btn);
        list_card.append(&list_header);

        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);
        scrolled.set_min_content_height(350);

        let server_list_box = ListBox::new();
        server_list_box.set_selection_mode(gtk4::SelectionMode::None);
        scrolled.set_child(Some(&server_list_box));
        list_card.append(&scrolled);

        container.append(&list_card);

        let servers = Rc::new(RefCell::new(MultiplayerManager::curated_servers()));

        let view = Self {
            container,
            server_list_box,
            refresh_btn,
            add_server_btn,
            server_name_entry,
            server_addr_entry,
            status_label,
            servers,
        };

        view.populate_servers();
        view
    }

    pub fn populate_servers(&self) {
        while let Some(child) = self.server_list_box.first_child() {
            self.server_list_box.remove(&child);
        }

        let servers = self.servers.borrow();
        for s in servers.iter() {
            let row = ListBoxRow::new();
            let row_box = GtkBox::new(Orientation::Horizontal, 12);
            row_box.set_css_classes(&["card"]);
            row_box.set_margin_bottom(6);

            let icon_lbl = Label::new(Some("🏰"));
            icon_lbl.set_css_classes(&["page-title"]);
            row_box.append(&icon_lbl);

            let col = GtkBox::new(Orientation::Vertical, 2);
            col.set_hexpand(true);

            let header_row = GtkBox::new(Orientation::Horizontal, 8);
            let name_lbl = Label::new(Some(&s.name));
            name_lbl.set_css_classes(&["card-title"]);
            name_lbl.set_halign(gtk4::Align::Start);
            header_row.append(&name_lbl);

            let cat_badge = Label::new(Some(&s.category));
            cat_badge.set_css_classes(&["badge-turbo"]);
            header_row.append(&cat_badge);

            let ver_badge = Label::new(Some(&s.supported_version));
            ver_badge.set_css_classes(&["badge-ready"]);
            header_row.append(&ver_badge);

            col.append(&header_row);

            let addr_lbl = Label::new(Some(&format!("{}:{}", s.address, s.port)));
            addr_lbl.set_css_classes(&["page-subtitle"]);
            addr_lbl.set_halign(gtk4::Align::Start);
            col.append(&addr_lbl);

            let desc_lbl = Label::new(Some(&s.description));
            desc_lbl.set_css_classes(&["page-subtitle"]);
            desc_lbl.set_halign(gtk4::Align::Start);
            desc_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
            col.append(&desc_lbl);

            row_box.append(&col);

            let ping_btn = Button::with_label("⚡ Ping");
            ping_btn.set_css_classes(&["btn-amber"]);
            ping_btn.set_valign(gtk4::Align::Center);

            let s_addr = s.address.clone();
            let s_port = s.port;
            let status_ref = self.status_label.clone();
            let ping_btn_clone = ping_btn.clone();

            ping_btn.connect_clicked(move |_| {
                let res: ServerPingResult = MultiplayerManager::ping_server(&s_addr, s_port);
                if res.is_online {
                    ping_btn_clone.set_label(&format!("{} ms", res.latency_ms.unwrap_or(0)));
                    ping_btn_clone.set_css_classes(&["btn-emerald"]);
                    status_ref.set_text(&format!(
                        "Realm {}:{} is online and responding.",
                        s_addr, s_port
                    ));
                    status_ref.set_css_classes(&["badge-ready"]);
                } else {
                    ping_btn_clone.set_label("Offline");
                    ping_btn_clone.set_css_classes(&["btn-danger"]);
                    status_ref.set_text(&format!(
                        "Realm {}:{} unreachable: {}",
                        s_addr, s_port, res.status_message
                    ));
                    status_ref.set_css_classes(&["badge-warning"]);
                }
            });

            row_box.append(&ping_btn);

            row.set_child(Some(&row_box));
            self.server_list_box.append(&row);
        }
    }
}
