use crate::i18n::t;
use crate::models::mime_type::get_categories;
use crate::ui::pages::page_builder::build_entry_page;

pub fn create_media_page() -> adw::PreferencesPage {
    let category = get_categories()
        .into_iter()
        .find(|c| c.id == "media")
        .expect("Media category not found");
    build_entry_page(&t("desc_media"), category.entries)
}
