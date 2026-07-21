# AGENTS.md

## Commands

```sh
cargo check                 # fast verification (no tests exist yet)
cargo build                 # debug build
cargo build --release       # release build
cargo fmt --check           # formatting lint
cargo clippy -- -D warnings # clippy lint (deny warnings)
make install                # install binary + .desktop to $PREFIX (/usr/local)
make run                    # cargo run
```

## System dependencies (build)

`libgtk-4-dev`, `libadwaita-1-dev`, `pkg-config`, `libappstream-dev` (for CI).

## Key architecture

- **Entrypoint:** `src/main.rs` → `src/application.rs` (`com.example.default-apps`) → `src/window.rs`
- **Layers:** `models/` → `services/` → `ui/` (widgets + pages) → `window.rs`
- **Backend:** Uses `gio::AppInfo` APIs exclusively (no manual `mimeapps.list` editing)
- **MIME types:** Tried in fallback order per entry (e.g. `audio/flac` → `audio/x-flac`). `get_default_for_types()` and `get_available_all()` in `services/default_apps.rs` handle multi-type lookup.
- **i18n:** Locale files in `locales/*.json`, embedded at compile time via `include_str!` in `src/i18n.rs`. Detects `LANG` env var. Use `crate::i18n::t("key")` for all user-facing strings.

## UI quirks

- **Row refresh after set-default** happens directly in the dialog callback via a captured `ActionRow` clone in `page_builder.rs`. No global refresh mechanism.
- **Search** switches the stack to the `"all"` page automatically. On clear, returns to the previously selected category. Sidebar clicks are ignored while search has text.
- **Crate aliases:** `gtk = { package = "gtk4" }`, `adw = { package = "libadwaita" }`. Use `gtk::*` and `adw::*` in code.
- **Feature flags:** `gtk/v4_14`, `adw/v1_5`, `gio/v2_80` — keep in sync with system library versions.

## Files of note

- `com.example.default-apps.desktop` — launcher, installs via `make install`
- `locales/*.json` — translations, add a new locale by creating the JSON and importing it in `src/i18n.rs`
