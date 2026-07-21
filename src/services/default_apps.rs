use gio::AppInfo;
use gio::prelude::*;

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

fn is_terminal_app(app: &AppInfo) -> bool {
    let id = app.id().map(|s| s.to_lowercase()).unwrap_or_default();
    let name = app.name().to_lowercase();
    let exec = app.executable().to_string_lossy().to_lowercase();

    id.contains("terminal")
        || id.contains("ptyxis")
        || id.contains("console")
        || id.contains("kitty")
        || id.contains("alacritty")
        || id.contains("konsole")
        || id.contains("xterm")
        || id.contains("foot")
        || id.contains("wezterm")
        || name.contains("terminal")
        || name.contains("term")
        || exec.ends_with("terminal")
        || exec.contains("ptyxis")
}

fn is_calculator_app(app: &AppInfo) -> bool {
    let id = app.id().map(|s| s.to_lowercase()).unwrap_or_default();
    let name = app.name().to_lowercase();
    let exec = app.executable().to_string_lossy().to_lowercase();

    id.contains("calculator")
        || id.contains("kcalc")
        || id.contains("galculator")
        || name.contains("calculator")
        || name.contains("calculadora")
        || exec.contains("calculator")
        || (exec.contains("calc") && !exec.contains("libreoffice"))
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

    if result.is_empty() {
        for mt in mime_types {
            let is_terminal = matches!(*mt, "application/x-terminal" | "x-scheme-handler/terminal");
            let is_calc = *mt == "application/x-calculator";

            if is_terminal || is_calc {
                for app in AppInfo::all() {
                    if !app.should_show() {
                        continue;
                    }
                    let matched = if is_terminal {
                        is_terminal_app(&app)
                    } else {
                        is_calculator_app(&app)
                    };

                    if matched {
                        let app_data = AppData::from(app);
                        if seen.insert(app_data.id.clone()) {
                            result.push(app_data);
                        }
                    }
                }
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
