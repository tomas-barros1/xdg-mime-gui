use gio::AppInfo;
use gio::prelude::AppInfoExt;

use crate::models::application::AppData;

pub fn get_default(mime_type: &str) -> Option<AppData> {
    AppInfo::default_for_type(mime_type, false).map(AppData::from)
}

pub fn get_default_for_types(mime_types: &[&str]) -> Option<AppData> {
    for mt in mime_types {
        if let Some(app) = get_default(mt) {
            return Some(app);
        }
    }
    None
}

pub fn get_available_all(mime_types: &[&str]) -> Vec<AppData> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for mt in mime_types {
        for app in AppInfo::all_for_type(mt)
            .into_iter()
            .filter(|app| app.should_show())
            .map(AppData::from)
        {
            if seen.insert(app.id.clone()) {
                result.push(app);
            }
        }
    }
    result
}

pub fn get_available(mime_type: &str) -> Vec<AppData> {
    AppInfo::all_for_type(mime_type)
        .into_iter()
        .filter(|app| app.should_show())
        .map(AppData::from)
        .collect()
}

pub fn set_default(app: &AppInfo, mime_type: &str) -> Result<(), glib::Error> {
    app.set_as_default_for_type(mime_type)
}
