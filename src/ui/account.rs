use crate::core::authentication::AccountProfile;
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, Entry, Label, Orientation, ProgressBar,
};

pub struct AccountView {
    pub container: GtkBox,
    pub profile_name_label: Label,
    pub profile_uuid_label: Label,
    pub profile_type_label: Label,
    pub ms_login_btn: Button,
    pub code_display_box: GtkBox,
    pub user_code_label: Label,
    pub open_browser_btn: Button,
    pub auth_progress: ProgressBar,
    pub auth_status_label: Label,
    pub offline_name_entry: Entry,
    pub set_offline_btn: Button,
    pub logout_btn: Button,
}

impl AccountView {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 16);
        container.set_css_classes(&["content-area"]);

        let title = Label::new(Some("ACCOUNT MANAGEMENT"));
        title.set_css_classes(&["page-title"]);
        title.set_halign(gtk4::Align::Start);

        let subtitle = Label::new(Some("Microsoft OAuth authentication & local personal profiles"));
        subtitle.set_css_classes(&["page-subtitle"]);
        subtitle.set_halign(gtk4::Align::Start);

        container.append(&title);
        container.append(&subtitle);

        // Active Profile Card
        let active_card = GtkBox::new(Orientation::Vertical, 10);
        active_card.set_css_classes(&["card"]);

        let a_title = Label::new(Some("ACTIVE PLAYER PROFILE"));
        a_title.set_css_classes(&["card-title"]);
        a_title.set_halign(gtk4::Align::Start);
        active_card.append(&a_title);

        let row1 = GtkBox::new(Orientation::Horizontal, 12);
        let name_title = Label::new(Some("Username:"));
        name_title.set_css_classes(&["page-subtitle"]);
        let profile_name_label = Label::new(Some("Player"));
        profile_name_label.set_css_classes(&["card-title"]);
        row1.append(&name_title);
        row1.append(&profile_name_label);

        let profile_type_label = Label::new(Some("LOCAL OFFLINE"));
        profile_type_label.set_css_classes(&["badge-info"]);
        row1.append(&profile_type_label);
        active_card.append(&row1);

        let row2 = GtkBox::new(Orientation::Horizontal, 12);
        let uuid_title = Label::new(Some("UUID:"));
        uuid_title.set_css_classes(&["page-subtitle"]);
        let profile_uuid_label = Label::new(Some("offline-uuid"));
        profile_uuid_label.set_css_classes(&["page-subtitle"]);
        row2.append(&uuid_title);
        row2.append(&profile_uuid_label);
        active_card.append(&row2);

        let logout_btn = Button::with_label("Logout / Switch Account");
        logout_btn.set_css_classes(&["btn-secondary"]);
        logout_btn.set_margin_top(8);
        active_card.append(&logout_btn);

        container.append(&active_card);

        // Microsoft OAuth Card
        let ms_card = GtkBox::new(Orientation::Vertical, 12);
        ms_card.set_css_classes(&["card"]);

        let ms_title = Label::new(Some("MICROSOFT OAUTH AUTHENTICATION"));
        ms_title.set_css_classes(&["card-title"]);
        ms_title.set_halign(gtk4::Align::Start);
        ms_card.append(&ms_title);

        let ms_desc = Label::new(Some(
            "Authenticate with your official Microsoft Minecraft account using the secure Device Code Flow without entering passwords into the launcher."
        ));
        ms_desc.set_css_classes(&["page-subtitle"]);
        ms_desc.set_wrap(true);
        ms_desc.set_halign(gtk4::Align::Start);
        ms_card.append(&ms_desc);

        let ms_login_btn = Button::with_label("🔐  Login with Microsoft");
        ms_login_btn.set_css_classes(&["btn-primary"]);
        ms_card.append(&ms_login_btn);

        // Device Code Box (hidden until login starts)
        let code_display_box = GtkBox::new(Orientation::Vertical, 8);
        code_display_box.set_visible(false);

        let instruct_lbl = Label::new(Some("1. Enter this code on microsoft.com/devicelogin:"));
        instruct_lbl.set_halign(gtk4::Align::Start);
        code_display_box.append(&instruct_lbl);

        let user_code_label = Label::new(Some("CODE-HERE"));
        user_code_label.set_css_classes(&["card-title"]);
        user_code_label.set_selectable(true);
        code_display_box.append(&user_code_label);

        let open_browser_btn = Button::with_label("🌐 Open https://microsoft.com/devicelogin");
        open_browser_btn.set_css_classes(&["btn-secondary"]);
        code_display_box.append(&open_browser_btn);

        let auth_progress = ProgressBar::new();
        auth_progress.set_pulse_step(0.1);
        code_display_box.append(&auth_progress);

        let auth_status_label = Label::new(Some("Waiting for user authorization in browser..."));
        auth_status_label.set_css_classes(&["page-subtitle"]);
        code_display_box.append(&auth_status_label);

        ms_card.append(&code_display_box);
        container.append(&ms_card);

        // Local Player Profile Card
        let offline_card = GtkBox::new(Orientation::Vertical, 10);
        offline_card.set_css_classes(&["card"]);

        let off_title = Label::new(Some("LOCAL / OFFLINE PROFILE"));
        off_title.set_css_classes(&["card-title"]);
        off_title.set_halign(gtk4::Align::Start);
        offline_card.append(&off_title);

        let off_desc = Label::new(Some(
            "For personal offline play or local LAN servers. Sets custom player nickname with deterministic offline UUID."
        ));
        off_desc.set_css_classes(&["page-subtitle"]);
        off_desc.set_wrap(true);
        off_desc.set_halign(gtk4::Align::Start);
        offline_card.append(&off_desc);

        let off_row = GtkBox::new(Orientation::Horizontal, 12);
        let offline_name_entry = Entry::new();
        offline_name_entry.set_placeholder_text(Some("Enter Nickname (e.g. alz)"));
        offline_name_entry.set_hexpand(true);
        off_row.append(&offline_name_entry);

        let set_offline_btn = Button::with_label("Set Local Profile");
        set_offline_btn.set_css_classes(&["btn-secondary"]);
        off_row.append(&set_offline_btn);
        offline_card.append(&off_row);

        container.append(&offline_card);

        Self {
            container,
            profile_name_label,
            profile_uuid_label,
            profile_type_label,
            ms_login_btn,
            code_display_box,
            user_code_label,
            open_browser_btn,
            auth_progress,
            auth_status_label,
            offline_name_entry,
            set_offline_btn,
            logout_btn,
        }
    }

    pub fn update_profile(&self, profile: &AccountProfile) {
        self.profile_name_label.set_text(&profile.username);
        self.profile_uuid_label.set_text(&profile.uuid);
        if profile.is_microsoft {
            self.profile_type_label.set_text("MICROSOFT MSA");
            self.profile_type_label.set_css_classes(&["badge-ready"]);
        } else {
            self.profile_type_label.set_text("LOCAL OFFLINE");
            self.profile_type_label.set_css_classes(&["badge-info"]);
        }
    }
}
