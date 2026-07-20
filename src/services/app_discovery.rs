use gio::AppInfo;
use gio::prelude::AppInfoExt;

use crate::models::application::AppData;

pub fn discover_all() -> Vec<AppData> {
    AppInfo::all()
        .into_iter()
        .filter(|app| app.should_show())
        .map(AppData::from)
        .collect()
}
