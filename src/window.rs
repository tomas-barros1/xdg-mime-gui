use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use adw::{NavigationPage, NavigationSplitView, ToastOverlay, ToolbarView};
use gtk::Stack;

use crate::i18n::t;
use crate::models::mime_type::{get_categories, get_entry_by_id, get_mime_type_description};
use crate::ui::pages::{all, internet, media, office, system};
use crate::ui::widgets::search::create_search_entry;

struct RowData {
    row: adw::ActionRow,
    keywords: String,
}

struct PageState {
    rows: Vec<RowData>,
}

pub fn build_window(app: &adw::Application) -> adw::ApplicationWindow {
    let window = adw::ApplicationWindow::new(app);
    window.set_title(Some(&t("window_title")));
    window.set_default_size(900, 650);

    let toast_overlay = ToastOverlay::new();

    let toolbar_view = ToolbarView::new();

    let header = adw::HeaderBar::new();
    toolbar_view.add_top_bar(&header);

    let search_entry = create_search_entry();
    search_entry.set_margin_start(12);
    search_entry.set_margin_end(12);
    search_entry.set_margin_top(6);
    search_entry.set_margin_bottom(6);
    toolbar_view.add_top_bar(&search_entry);

    let split_view = NavigationSplitView::new();
    split_view.set_vexpand(true);

    let sidebar_list = gtk::ListBox::new();
    sidebar_list.add_css_class("navigation-sidebar");
    sidebar_list.set_selection_mode(gtk::SelectionMode::Single);

    let categories = get_categories();
    let mut sidebar_map: Vec<(String, u32)> = Vec::new();
    let mut page_index: u32 = 0;

    for cat in &categories {
        let row = gtk::ListBoxRow::new();
        let label = gtk::Label::new(Some(&t(cat.name)));
        label.set_margin_start(12);
        label.set_margin_end(12);
        label.set_margin_top(8);
        label.set_margin_bottom(8);
        label.set_halign(gtk::Align::Start);
        row.set_child(Some(&label));
        sidebar_list.append(&row);
        sidebar_map.push((cat.id.to_string(), page_index));
        page_index += 1;
    }

    let all_row = gtk::ListBoxRow::new();
    let all_label = gtk::Label::new(Some(&t("All")));
    all_label.set_margin_start(12);
    all_label.set_margin_end(12);
    all_label.set_margin_top(8);
    all_label.set_margin_bottom(8);
    all_label.set_halign(gtk::Align::Start);
    all_row.set_child(Some(&all_label));
    sidebar_list.append(&all_row);
    sidebar_map.push(("all".to_string(), page_index));

    let sidebar_page = NavigationPage::new(&sidebar_list, &t("sidebar_title"));
    split_view.set_sidebar(Some(&sidebar_page));

    let stack = Stack::new();
    stack.set_vexpand(true);

    let page_states: Rc<RefCell<std::collections::HashMap<String, PageState>>> =
        Rc::new(RefCell::new(std::collections::HashMap::new()));

    for (cat_id, _) in &sidebar_map {
        let page: adw::PreferencesPage = if cat_id == "all" {
            all::create_all_page()
        } else if cat_id == "internet" {
            internet::create_internet_page()
        } else if cat_id == "media" {
            media::create_media_page()
        } else if cat_id == "office" {
            office::create_office_page()
        } else if cat_id == "system" {
            system::create_system_page()
        } else {
            continue;
        };

        let rows = collect_rows(&page);
        let page_widget = page.upcast_ref::<gtk::Widget>().clone();
        stack.add_titled(&page_widget, Some(cat_id.as_str()), cat_id);
        page_states
            .borrow_mut()
            .insert(cat_id.clone(), PageState { rows });
    }

    let content_page = NavigationPage::new(&stack, &t("window_title"));
    split_view.set_content(Some(&content_page));

    toolbar_view.set_content(Some(&split_view));
    toast_overlay.set_child(Some(&toolbar_view));
    window.set_content(Some(&toast_overlay));

    if let Some(first_row) = sidebar_list.first_child()
        && let Ok(row) = first_row.downcast::<gtk::ListBoxRow>()
    {
        sidebar_list.select_row(Some(&row));
    }
    stack.set_visible_child_name("internet");

    let last_category: Rc<RefCell<String>> = Rc::new(RefCell::new("internet".to_string()));

    {
        let stack = stack.clone();
        let page_states = page_states.clone();
        let search_entry = search_entry.clone();
        let sidebar_map = sidebar_map.clone();
        let last_category = last_category.clone();
        sidebar_list.connect_row_selected(move |_list, maybe_row| {
            let text = search_entry.text().to_string();
            if !text.trim().is_empty() {
                return;
            }
            if let Some(row) = maybe_row {
                let idx = row.index() as usize;
                if idx < sidebar_map.len() {
                    let cat_id = &sidebar_map[idx].0;
                    *last_category.borrow_mut() = cat_id.clone();
                    stack.set_visible_child_name(cat_id);
                    apply_search_filter(&stack, &page_states, "");
                }
            }
        });
    }

    {
        let stack = stack.clone();
        let page_states = page_states.clone();
        let last_category = last_category.clone();
        let sidebar_list = sidebar_list.clone();
        let sidebar_map = sidebar_map.clone();
        search_entry.connect_search_changed(move |entry| {
            let text = entry.text().to_string();
            if text.trim().is_empty() {
                let last = last_category.borrow().clone();
                stack.set_visible_child_name(&last);
                select_sidebar_category(&sidebar_list, &sidebar_map, &last);
            } else {
                stack.set_visible_child_name("all");
                select_sidebar_category(&sidebar_list, &sidebar_map, "all");
            }
            apply_search_filter(&stack, &page_states, &text);
        });
    }

    window
}

fn select_sidebar_category(
    sidebar_list: &gtk::ListBox,
    sidebar_map: &[(String, u32)],
    cat_id: &str,
) {
    if let Some(idx) = sidebar_map.iter().position(|(id, _)| id == cat_id)
        && let Some(row) = sidebar_list.row_at_index(idx as i32)
    {
        sidebar_list.select_row(Some(&row));
    }
}

fn collect_rows(page: &adw::PreferencesPage) -> Vec<RowData> {
    let mut rows = Vec::new();
    collect_rows_recursive(page.upcast_ref::<gtk::Widget>(), &mut rows);
    rows
}

fn collect_rows_recursive(widget: &gtk::Widget, rows: &mut Vec<RowData>) {
    if let Ok(row) = widget.clone().downcast::<adw::ActionRow>() {
        let widget_name = row.widget_name();
        let keywords = if let Some(entry) = get_entry_by_id(widget_name.as_str()) {
            let desc_list: Vec<String> = entry
                .mime_types
                .iter()
                .map(|m| get_mime_type_description(m))
                .collect();
            format!(
                "{} {} {} {}",
                t(entry.display_name),
                entry.display_name,
                entry.mime_types.join(" "),
                desc_list.join(" ")
            )
            .to_lowercase()
        } else {
            String::new()
        };
        rows.push(RowData { row, keywords });
    }
    let mut child = widget.first_child();
    while let Some(c) = child {
        collect_rows_recursive(&c, rows);
        child = c.next_sibling();
    }
}

fn apply_search_filter(
    stack: &Stack,
    page_states: &Rc<RefCell<std::collections::HashMap<String, PageState>>>,
    search_text: &str,
) {
    let visible = stack.visible_child_name().unwrap_or_default();
    let states = page_states.borrow();
    if search_text.trim().is_empty() {
        for state in states.values() {
            for row_data in &state.rows {
                row_data.row.set_visible(true);
            }
        }
    } else {
        let lower = search_text.trim().to_lowercase();
        for (name, state) in states.iter() {
            if name.as_str() == visible.as_str() {
                for row_data in &state.rows {
                    let title = row_data.row.title().to_lowercase();
                    let subtitle = row_data.row.subtitle().unwrap_or_default().to_lowercase();
                    let matches = title.contains(&lower)
                        || subtitle.contains(&lower)
                        || row_data.keywords.contains(&lower);
                    row_data.row.set_visible(matches);
                }
            } else {
                for row_data in &state.rows {
                    row_data.row.set_visible(false);
                }
            }
        }
    }
}
