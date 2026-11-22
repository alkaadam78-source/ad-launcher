use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation, Separator};

/// Creates a standard Dark Baroque Page Header with:
/// 1. Title and Subtitle in consistent classical typography
/// 2. Elegant creator cartouche badge
/// 3. Subtle ornamental separator beneath the heading (Rule 1)
pub fn create_page_header(title_text: &str, subtitle_text: &str) -> GtkBox {
    let header_container = GtkBox::new(Orientation::Vertical, 8);

    let row = GtkBox::new(Orientation::Horizontal, 14);

    let title_col = GtkBox::new(Orientation::Vertical, 2);
    let title = Label::new(Some(title_text));
    title.set_css_classes(&["page-title"]);
    title.set_halign(gtk4::Align::Start);

    let subtitle = Label::new(Some(subtitle_text));
    subtitle.set_css_classes(&["page-subtitle"]);
    subtitle.set_halign(gtk4::Align::Start);

    title_col.append(&title);
    title_col.append(&subtitle);
    row.append(&title_col);

    let credit_badge = Label::new(Some("Crafted by AdamDevnoob"));
    credit_badge.set_css_classes(&["adam-credit-badge"]);
    credit_badge.set_halign(gtk4::Align::End);
    credit_badge.set_hexpand(true);
    credit_badge.set_valign(gtk4::Align::Center);
    row.append(&credit_badge);

    header_container.append(&row);

    // Subtle ornamental separator beneath major headings (Rule 1)
    let sep = create_section_divider();
    header_container.append(&sep);

    header_container
}

/// Creates a thin antique-gold section divider with a small symmetrical Baroque ornament at center (Rule 2)
pub fn create_section_divider() -> GtkBox {
    let container = GtkBox::new(Orientation::Horizontal, 8);
    container.set_css_classes(&["ornamental-separator-box"]);
    container.set_halign(gtk4::Align::Fill);
    container.set_hexpand(true);
    container.set_margin_top(4);
    container.set_margin_bottom(6);

    // Left line
    let left_sep = Separator::new(Orientation::Horizontal);
    left_sep.set_css_classes(&["divider-line"]);
    left_sep.set_hexpand(true);
    left_sep.set_valign(gtk4::Align::Center);
    container.append(&left_sep);

    // Center symmetrical Baroque ornament
    let center_ornament = Label::new(Some("⚜"));
    center_ornament.set_css_classes(&["divider-ornament"]);
    center_ornament.set_valign(gtk4::Align::Center);
    container.append(&center_ornament);

    // Right line
    let right_sep = Separator::new(Orientation::Horizontal);
    right_sep.set_css_classes(&["divider-line"]);
    right_sep.set_hexpand(true);
    right_sep.set_valign(gtk4::Align::Center);
    container.append(&right_sep);

    container
}

/// Creates a tasteful empty state with subtle decorative framing and iconography (Rule 8)
pub fn create_empty_state(icon: &str, title: &str, subtitle: &str) -> GtkBox {
    let box_container = GtkBox::new(Orientation::Vertical, 8);
    box_container.set_css_classes(&["empty-state-card"]);
    box_container.set_halign(gtk4::Align::Center);
    box_container.set_valign(gtk4::Align::Center);
    box_container.set_margin_top(28);
    box_container.set_margin_bottom(28);

    let icon_lbl = Label::new(Some(icon));
    icon_lbl.set_css_classes(&["empty-state-icon"]);
    box_container.append(&icon_lbl);

    let title_lbl = Label::new(Some(title));
    title_lbl.set_css_classes(&["empty-state-title"]);
    box_container.append(&title_lbl);

    let sub_lbl = Label::new(Some(subtitle));
    sub_lbl.set_css_classes(&["empty-state-subtitle"]);
    box_container.append(&sub_lbl);

    box_container
}
