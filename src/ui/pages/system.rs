use crate::i18n::t;
use crate::models::mime_type::get_categories;
use crate::ui::pages::page_builder::build_entry_page;

pub fn create_system_page() -> adw::PreferencesPage {
    let category = get_categories()
        .into_iter()
        .find(|c| c.id == "system")
        .expect("System category not found");
    build_entry_page(&t("desc_system"), category.entries)
}
