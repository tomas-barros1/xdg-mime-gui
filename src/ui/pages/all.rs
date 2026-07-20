use crate::i18n::t;
use crate::models::mime_type::get_all_entries;
use crate::ui::pages::page_builder::build_entry_page;

pub fn create_all_page() -> adw::PreferencesPage {
    build_entry_page(&t("desc_all"), get_all_entries())
}
