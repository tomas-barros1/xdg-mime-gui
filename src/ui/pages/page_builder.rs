use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup, PreferencesPage};
use gtk::Image;

use crate::i18n::t;
use crate::models::mime_type::CategoryEntry;
use crate::services::default_apps::get_default_for_types;
use crate::ui::widgets::app_dialog::show_app_selection_dialog;

use crate::services::observer::EntryObserverBus;

pub fn build_entry_page(description: &str, entries: Vec<CategoryEntry>) -> PreferencesPage {
    let page = PreferencesPage::new();
    let group = PreferencesGroup::new();
    group.set_description(Some(description));

    for entry in entries {
        let row = create_entry_row(entry);
        group.add(&row);
    }

    page.add(&group);
    page
}

fn create_entry_row(entry: CategoryEntry) -> ActionRow {
    let mime_types = entry.mime_types.clone();
    let current_default = get_default_for_types(&mime_types);

    let row = ActionRow::new();
    row.set_widget_name(entry.id);
    row.set_title(&t(entry.display_name));
    let subtitle = match &current_default {
        Some(app) => app.name.clone(),
        None => t("not_set"),
    };
    row.set_subtitle(&subtitle);

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

    // Register observer for this entry_id so all instances of this row across all pages update automatically
    let row_for_observer = row.clone();
    let mime_types_for_observer = mime_types.clone();
    EntryObserverBus::global().subscribe(entry.id, move || {
        if let Some(default) = get_default_for_types(&mime_types_for_observer) {
            row_for_observer.set_subtitle(&default.name);
        } else {
            row_for_observer.set_subtitle(&t("not_set"));
        }
    });

    let entry_id = entry.id;
    button.connect_clicked(move |btn| {
        show_app_selection_dialog(btn, &entry, move |_app| {
            EntryObserverBus::global().notify(entry_id);
        });
    });

    row
}
