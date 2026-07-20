use std::collections::HashMap;
use std::sync::LazyLock;

use serde::Deserialize;

#[derive(Deserialize)]
struct LocaleData {
    #[serde(flatten)]
    messages: HashMap<String, String>,
}

fn parse_locale(data: &str) -> HashMap<String, String> {
    serde_json::from_str::<LocaleData>(data)
        .expect("Failed to parse locale file")
        .messages
}

static EN: LazyLock<HashMap<String, String>> =
    LazyLock::new(|| parse_locale(include_str!("../locales/en.json")));

static PT_BR: LazyLock<HashMap<String, String>> =
    LazyLock::new(|| parse_locale(include_str!("../locales/pt_BR.json")));

static LOCALE: LazyLock<String> = LazyLock::new(|| {
    let raw = std::env::var("LANG").unwrap_or_default();
    let locale = raw.split('.').next().unwrap_or("en_US");
    if locale.starts_with("pt_BR") {
        "pt_BR".to_string()
    } else {
        "en".to_string()
    }
});

pub fn t(key: &str) -> String {
    let map = match LOCALE.as_str() {
        "pt_BR" => &*PT_BR,
        _ => &*EN,
    };
    map.get(key).cloned().unwrap_or_else(|| key.to_string())
}

pub fn locale_name() -> String {
    LOCALE.clone()
}
