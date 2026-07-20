use gio::{content_type_get_description, content_type_get_mime_type};

pub fn get_description(mime_type: &str) -> String {
    let desc = content_type_get_description(mime_type);
    if desc.is_empty() {
        mime_type.to_string()
    } else {
        desc.to_string()
    }
}

pub fn get_mime_type_from_mime(mime: &str) -> String {
    content_type_get_mime_type(mime)
        .map(|s| s.to_string())
        .unwrap_or_else(|| mime.to_string())
}
