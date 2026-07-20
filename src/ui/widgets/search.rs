use gtk::SearchEntry;

use crate::i18n::t;

pub fn create_search_entry() -> SearchEntry {
    let entry = SearchEntry::new();
    entry.set_placeholder_text(Some(&t("search_placeholder")));
    entry
}
