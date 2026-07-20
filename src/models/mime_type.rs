#[derive(Clone, Debug)]
pub struct CategoryEntry {
    pub id: &'static str,
    pub display_name: &'static str,
    pub icon_name: &'static str,
    pub mime_types: Vec<&'static str>,
}

#[derive(Clone, Debug)]
pub struct Category {
    pub id: &'static str,
    pub name: &'static str,
    pub icon_name: &'static str,
    pub entries: Vec<CategoryEntry>,
}

pub fn get_categories() -> Vec<Category> {
    vec![
        Category {
            id: "internet",
            name: "Internet",
            icon_name: "network-server-symbolic",
            entries: internet_entries(),
        },
        Category {
            id: "media",
            name: "Media",
            icon_name: "multimedia-player-symbolic",
            entries: media_entries(),
        },
        Category {
            id: "office",
            name: "Office",
            icon_name: "x-office-document-symbolic",
            entries: office_entries(),
        },
        Category {
            id: "system",
            name: "System",
            icon_name: "computer-symbolic",
            entries: system_entries(),
        },
    ]
}

pub fn get_all_entries() -> Vec<CategoryEntry> {
    let mut all = Vec::new();
    for cat in get_categories() {
        for entry in cat.entries {
            all.push(entry);
        }
    }
    all
}

fn internet_entries() -> Vec<CategoryEntry> {
    vec![
        CategoryEntry {
            id: "web-browser",
            display_name: "Web Browser",
            icon_name: "web-browser-symbolic",
            mime_types: vec![
                "x-scheme-handler/http",
                "x-scheme-handler/https",
                "text/html",
            ],
        },
        CategoryEntry {
            id: "email-client",
            display_name: "Email Client",
            icon_name: "mail-send-symbolic",
            mime_types: vec!["x-scheme-handler/mailto", "message/rfc822"],
        },
        CategoryEntry {
            id: "torrent-client",
            display_name: "Torrent Client",
            icon_name: "network-server-symbolic",
            mime_types: vec!["x-scheme-handler/magnet", "application/x-bittorrent"],
        },
        CategoryEntry {
            id: "rss-reader",
            display_name: "RSS Reader",
            icon_name: "internet-news-reader-symbolic",
            mime_types: vec!["application/rss+xml"],
        },
    ]
}

fn media_entries() -> Vec<CategoryEntry> {
    vec![
        CategoryEntry {
            id: "music-player",
            display_name: "Music Player",
            icon_name: "audio-x-generic-symbolic",
            mime_types: vec![
                "audio/mpeg",
                "audio/ogg",
                "audio/flac",
                "audio/x-flac",
                "audio/x-wav",
            ],
        },
        CategoryEntry {
            id: "video-player",
            display_name: "Video Player",
            icon_name: "video-x-generic-symbolic",
            mime_types: vec!["video/mp4", "video/mpeg", "video/x-matroska", "video/webm"],
        },
        CategoryEntry {
            id: "image-viewer",
            display_name: "Image Viewer",
            icon_name: "image-x-generic-symbolic",
            mime_types: vec!["image/png", "image/jpeg", "image/gif", "image/webp"],
        },
        CategoryEntry {
            id: "gif-viewer",
            display_name: "GIF Viewer",
            icon_name: "image-x-generic-symbolic",
            mime_types: vec!["image/gif"],
        },
        CategoryEntry {
            id: "svg-viewer",
            display_name: "SVG Viewer",
            icon_name: "image-x-generic-symbolic",
            mime_types: vec!["image/svg+xml"],
        },
    ]
}

fn office_entries() -> Vec<CategoryEntry> {
    vec![
        CategoryEntry {
            id: "pdf-viewer",
            display_name: "PDF Viewer",
            icon_name: "x-office-document-symbolic",
            mime_types: vec!["application/pdf"],
        },
        CategoryEntry {
            id: "document-viewer",
            display_name: "Document Viewer",
            icon_name: "x-office-document-symbolic",
            mime_types: vec![
                "application/msword",
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            ],
        },
        CategoryEntry {
            id: "spreadsheet-viewer",
            display_name: "Spreadsheet Viewer",
            icon_name: "x-office-spreadsheet-symbolic",
            mime_types: vec![
                "application/vnd.ms-excel",
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            ],
        },
        CategoryEntry {
            id: "presentation-viewer",
            display_name: "Presentation Viewer",
            icon_name: "x-office-presentation-symbolic",
            mime_types: vec![
                "application/vnd.ms-powerpoint",
                "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            ],
        },
    ]
}

fn system_entries() -> Vec<CategoryEntry> {
    vec![
        CategoryEntry {
            id: "file-manager",
            display_name: "File Manager",
            icon_name: "folder-symbolic",
            mime_types: vec!["inode/directory"],
        },
        CategoryEntry {
            id: "terminal-emulator",
            display_name: "Terminal Emulator",
            icon_name: "utilities-terminal-symbolic",
            mime_types: vec!["application/x-terminal", "x-scheme-handler/terminal"],
        },
        CategoryEntry {
            id: "text-editor",
            display_name: "Text Editor",
            icon_name: "accessories-text-editor-symbolic",
            mime_types: vec!["text/plain"],
        },
        CategoryEntry {
            id: "calculator",
            display_name: "Calculator",
            icon_name: "accessories-calculator-symbolic",
            mime_types: vec!["application/x-calculator"],
        },
    ]
}

pub fn get_entry_by_id(id: &str) -> Option<CategoryEntry> {
    get_all_entries().into_iter().find(|e| e.id == id)
}

pub fn get_mime_type_description(mime: &str) -> String {
    match mime {
        "x-scheme-handler/http" => "HTTP".into(),
        "x-scheme-handler/https" => "HTTPS".into(),
        "x-scheme-handler/mailto" => "Mailto".into(),
        "x-scheme-handler/magnet" => "Magnet".into(),
        "x-scheme-handler/terminal" => "Terminal".into(),
        "message/rfc822" => "Email".into(),
        "application/pdf" => "PDF Document".into(),
        "application/rss+xml" => "RSS Feed".into(),
        "application/x-bittorrent" => "Torrent".into(),
        "application/msword" => "Word Document".into(),
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
            "Word Document".into()
        }
        "application/vnd.ms-excel" => "Excel Spreadsheet".into(),
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => {
            "Excel Spreadsheet".into()
        }
        "application/vnd.ms-powerpoint" => "PowerPoint Presentation".into(),
        "application/vnd.openxmlformats-officedocument.presentationml.presentation" => {
            "PowerPoint Presentation".into()
        }
        "application/x-terminal" => "Terminal".into(),
        "application/x-calculator" => "Calculator".into(),
        "inode/directory" => "Directory".into(),
        "text/plain" => "Text File".into(),
        "text/html" => "HTML File".into(),
        "audio/mpeg" => "MP3 Audio".into(),
        "audio/ogg" => "OGG Audio".into(),
        "audio/flac" => "FLAC Audio".into(),
        "audio/x-flac" => "FLAC Audio".into(),
        "audio/x-wav" => "WAV Audio".into(),
        "video/mp4" => "MP4 Video".into(),
        "video/mpeg" => "MPEG Video".into(),
        "video/x-matroska" => "Matroska Video".into(),
        "video/webm" => "WebM Video".into(),
        "image/png" => "PNG Image".into(),
        "image/jpeg" => "JPEG Image".into(),
        "image/gif" => "GIF Image".into(),
        "image/webp" => "WebP Image".into(),
        "image/svg+xml" => "SVG Image".into(),
        _ => mime.to_string(),
    }
}
