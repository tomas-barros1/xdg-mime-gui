use adw::ActionRow;
use adw::prelude::*;
use gtk::Image;

use crate::models::mime_type::CategoryEntry;

pub fn create_app_row(
    entry: &CategoryEntry,
    current_default_name: Option<String>,
    on_clicked: impl Fn() + 'static,
) -> ActionRow {
    let row = ActionRow::new();
    row.set_title(entry.display_name);

    let subtitle = current_default_name.as_deref().unwrap_or("Not set");
    row.set_subtitle(subtitle);

    let icon = Image::from_icon_name(entry.icon_name);
    icon.add_css_class("symbolic");
    icon.set_pixel_size(32);
    row.add_prefix(&icon);

    let button = gtk::Button::new();
    button.set_icon_name("go-next-symbolic");
    button.set_valign(gtk::Align::Center);
    button.add_css_class("flat");
    row.add_suffix(&button);
    row.set_activatable_widget(Some(&button));

    button.connect_clicked(move |_| on_clicked());

    row
}
