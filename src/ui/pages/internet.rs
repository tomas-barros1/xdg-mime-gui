use crate::i18n::t;
use crate::models::mime_type::get_categories;
use crate::ui::pages::page_builder::build_entry_page;

pub fn create_internet_page() -> adw::PreferencesPage {
    let category = get_categories()
        .into_iter()
        .find(|c| c.id == "internet")
        .expect("Internet category not found");
    build_entry_page(&t("desc_internet"), category.entries)
}
