use gio::prelude::AppInfoExt;
use gio::{AppInfo, Icon};
use glib::object::Cast;

#[derive(Clone, Debug)]
pub struct AppData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon_name: Option<String>,
    pub executable: String,
    pub gio_app: AppInfo,
}

impl From<AppInfo> for AppData {
    fn from(app: AppInfo) -> Self {
        let id = app
            .id()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".into());
        let name = app.display_name().to_string();
        let description = app.description().map(|s| s.to_string()).unwrap_or_default();
        let icon_name = app.icon().and_then(|icon| icon_name_from_gicon(&icon));
        let executable = app.executable().to_string_lossy().to_string();

        AppData {
            id,
            name,
            description,
            icon_name,
            executable,
            gio_app: app,
        }
    }
}

pub fn icon_name_from_gicon(icon: &Icon) -> Option<String> {
    icon.dynamic_cast_ref::<gio::ThemedIcon>()
        .and_then(|themed| themed.names().first().map(|s| s.to_string()))
        .or_else(|| icon.dynamic_cast_ref::<gio::FileIcon>().and(None))
}
