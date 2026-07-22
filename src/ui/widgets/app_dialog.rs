use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use gio::AppInfo;
use gio::prelude::AppInfoExt;

use crate::i18n::t;
use crate::models::application::AppData;
use crate::models::mime_type::CategoryEntry;
use crate::services::default_apps::{get_available_all, get_default_for_types, set_default};

pub fn show_app_selection_dialog(
    parent: &impl IsA<gtk::Widget>,
    entry: &CategoryEntry,
    on_set: impl Fn(AppInfo) + 'static,
) {
    let dialog = gtk::Window::new();
    dialog.set_title(Some(entry.display_name));
    dialog.set_modal(true);
    dialog.set_default_size(400, 500);

    if let Some(root) = parent.root()
        && let Some(window) = root.downcast_ref::<gtk::Window>()
    {
        dialog.set_transient_for(Some(window));
    }

    let all_types = entry.mime_types.to_vec();
    let current_default = get_default_for_types(&all_types);
    let available = get_available_all(&all_types);

    let selected_app: Rc<RefCell<Option<AppData>>> = Rc::new(RefCell::new(current_default.clone()));

    let content = gtk::Box::new(gtk::Orientation::Vertical, 12);
    content.set_margin_start(24);
    content.set_margin_end(24);
    content.set_margin_top(24);
    content.set_margin_bottom(24);

    let title_label = gtk::Label::new(Some(&t(entry.display_name)));
    title_label.add_css_class("title-2");
    title_label.set_halign(gtk::Align::Start);

    let current_label = gtk::Label::new(Some(&t("dialog_current")));
    current_label.add_css_class("heading");
    current_label.set_halign(gtk::Align::Start);

    let current_name = match &current_default {
        Some(a) => a.name.clone(),
        None => t("dialog_none"),
    };
    let current_badge = gtk::Label::new(Some(&current_name));
    current_badge.add_css_class("accent");
    current_badge.set_halign(gtk::Align::Start);
    current_badge.set_margin_start(12);

    let available_label = gtk::Label::new(Some(&t("dialog_available")));
    available_label.add_css_class("heading");
    available_label.set_halign(gtk::Align::Start);

    let list_box = gtk::ListBox::new();
    list_box.add_css_class("boxed-list");

    let mut first_radio: Option<gtk::CheckButton> = None;

    for app in &available {
        let row = gtk::ListBoxRow::new();
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        hbox.set_margin_start(12);
        hbox.set_margin_end(12);
        hbox.set_margin_top(8);
        hbox.set_margin_bottom(8);

        let app_icon = gtk::Image::new();
        if let Some(icon) = app.gio_app.icon() {
            app_icon.set_from_gicon(&icon);
        } else {
            app_icon.set_icon_name(Some("application-x-executable-symbolic"));
        }
        app_icon.set_pixel_size(32);

        let name_label = gtk::Label::new(Some(&app.name));
        name_label.set_halign(gtk::Align::Start);
        name_label.set_hexpand(true);

        let radio = gtk::CheckButton::new();

        if let Some(ref first) = first_radio {
            radio.set_group(Some(first));
        } else {
            first_radio = Some(radio.clone());
        }

        let is_current = current_default.as_ref().is_some_and(|c| c.id == app.id);
        if is_current {
            radio.set_active(true);
        }

        {
            let selected_app = selected_app.clone();
            let app_data = app.clone();
            radio.connect_toggled(move |r| {
                if r.is_active() {
                    *selected_app.borrow_mut() = Some(app_data.clone());
                }
            });
        }

        hbox.append(&app_icon);
        hbox.append(&name_label);
        hbox.append(&radio);
        row.set_child(Some(&hbox));
        list_box.append(&row);
    }

    let scrolled = gtk::ScrolledWindow::new();
    scrolled.set_child(Some(&list_box));
    scrolled.set_vexpand(true);

    let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    button_box.set_halign(gtk::Align::End);

    let cancel_button = gtk::Button::with_label(&t("dialog_cancel"));
    cancel_button.connect_clicked({
        let dialog = dialog.clone();
        move |_| dialog.close()
    });

    let set_button = gtk::Button::with_label(&t("dialog_set_default"));
    set_button.add_css_class("suggested-action");

    set_button.connect_clicked({
        let dialog = dialog.clone();
        let mime_types = entry.mime_types.clone();
        move |_| {
            let app_data = selected_app.borrow().clone();
            if let Some(app_data) = app_data {
                let mut success = false;
                for mt in &mime_types {
                    if set_default(&app_data.gio_app, mt).is_ok() {
                        success = true;
                    }
                }
                if success {
                    on_set(app_data.gio_app.clone());
                }
            }
            dialog.close();
        }
    });

    button_box.append(&cancel_button);
    button_box.append(&set_button);

    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);

    content.append(&title_label);
    content.append(&separator);
    content.append(&current_label);
    content.append(&current_badge);
    content.append(&available_label);
    content.append(&scrolled);
    content.append(&button_box);

    dialog.set_child(Some(&content));
    dialog.present();
}
