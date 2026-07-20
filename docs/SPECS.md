# Default Apps Manager

## Overview

Create a beautiful, modern Linux desktop application built with **Rust + GTK4 + libadwaita** that allows users to easily manage their system default applications.

The goal is to provide a simple, polished alternative to manually editing:

- `mimeapps.list`
- `.desktop` files
- `xdg-mime`
- `xdg-settings`

The application should feel native on GNOME desktops while remaining compatible with other GTK-based environments.

---

# Project Name

Default Apps

Possible names:

- App Defaults
- Default Manager
- Application Chooser

---

# Technology Stack

## Required

- Rust
- GTK4
- libadwaita
- GLib
- GIO

## Linux Integration

Use:

- `gio::AppInfo`
- MIME database APIs
- XDG desktop application standards

Avoid manually editing configuration files unless absolutely necessary.

---

# Design Goals

The application must be:

- Beautiful
- Simple
- Fast
- Beginner friendly
- Native looking
- Keyboard accessible

Design inspiration:

- GNOME Settings
- GNOME Software
- Nautilus
- Loupe

Follow GNOME design principles:

- large spacing
- rounded corners
- clean typography
- minimal interface
- clear hierarchy

---

# Main Window

The main window should contain:

```
┌────────────────────────────────────────────┐
│ Default Applications                       │
├────────────────────────────────────────────┤
│ 🔍 Search applications                     │
├────────────────────────────────────────────┤
│                                            │
│ Web Browser                 Firefox     >  │
│ Email Client                Thunderbird >  │
│ PDF Viewer                  Evince      >  │
│ Music Player                Rhythmbox   >  │
│ Video Player                VLC         >  │
│ Image Viewer                Loupe       >  │
│ File Manager                Nautilus    >  │
│ Terminal                    Ptyxis      >  │
│ Text Editor                 Gedit       >  │
│                                            │
└────────────────────────────────────────────┘
```

---

# Categories

## Internet

Manage:

- Web Browser
- Email Client
- Torrent Client
- RSS Reader

---

## Media

Manage:

- Music Player
- Video Player
- Image Viewer
- GIF Viewer
- SVG Viewer

---

## Office

Manage:

- PDF Viewer
- Document Viewer
- Spreadsheet Viewer
- Presentation Viewer

---

## System

Manage:

- File Manager
- Terminal Emulator
- Text Editor
- Calculator

---

## Advanced MIME Management

Allow users to manage any MIME association.

Examples:

```
application/pdf
image/png
image/jpeg
video/mp4
audio/mpeg
text/plain
```

---

# Application List

Each application entry should look like:

```
┌─────────────────────────────────────┐
│                                     │
│  🦊 Firefox                         │
│     Default Web Browser             │
│                                     │
│                         Firefox  >  │
└─────────────────────────────────────┘
```

Use:

- `AdwActionRow`
- application icon
- application name
- description
- current default application

---

# Application Selection

When clicking an application category:

Open a selection dialog:

```
PDF Viewer

Current application:

✓ Evince


Available applications:

○ Okular

○ Firefox

○ Chrome

○ LibreOffice Draw


[Cancel]     [Set Default]
```

---

# Application Details

Allow viewing:

```
Application Name

Icon

Desktop ID

Executable Command

Version

Categories

Supported MIME Types
```

---

# Backend

Use:

```rust
gio::AppInfo
```

for:

- listing installed applications
- detecting default applications
- changing default applications

Example:

```rust
AppInfo::all()

AppInfo::default_for_type()

AppInfo::set_as_default_for_type()
```

---

# Application Discovery

Load applications using:

```rust
gio::AppInfo::all()
```

Filter:

Remove:

```
NoDisplay=true

Hidden=true
```

Only show valid desktop applications.

---

# Icons

Use:

```rust
gtk::Image::from_gicon()
```

Never use absolute icon paths.

Support:

- system icons
- application icons
- fallback icons

---

# Search System

Implement instant search.

Use:

```rust
gtk::SearchEntry
```

Search by:

- application name
- description
- MIME type
- extension

Example:

```
Search:

pdf
```

Results:

```
Evince

Okular

Firefox

Chrome
```

---

# Navigation

Use:

```rust
AdwNavigationSplitView
```

Structure:

```
ApplicationWindow

 ├── HeaderBar
 │
 ├── SearchEntry
 │
 └── NavigationSplitView

       ├── Sidebar

       │    ├── Internet
       │    ├── Media
       │    ├── Office
       │    ├── System
       │    └── All

       └── Content View
```

---

# Responsive Design

Small screens:

- hide sidebar
- use navigation stack

Large screens:

- persistent sidebar

Tablet:

- collapsible navigation

---

# Theme Support

Support:

- Light mode
- Dark mode
- High contrast mode

Use libadwaita styling.

Avoid excessive CSS.

Only customize:

- spacing
- padding
- small visual improvements

---

# Accessibility

Required:

- keyboard navigation
- screen reader support
- visible focus indicators
- proper labels
- tooltips
- GTK shortcuts

---

# Performance Requirements

The application should:

- start quickly
- avoid blocking the UI thread
- cache application information
- update only changed rows
- use lazy loading when possible

Use:

- async tasks
- GTK models
- reactive bindings

---

# Project Structure

```
src/

├── main.rs
├── application.rs
├── window.rs
│
├── models/
│   ├── application.rs
│   └── mime_type.rs
│
├── services/
│   ├── app_discovery.rs
│   ├── default_apps.rs
│   └── mime_database.rs
│
└── ui/

    ├── pages/

    │   ├── internet.rs
    │   ├── media.rs
    │   ├── office.rs
    │   ├── system.rs
    │   └── all.rs
    │
    └── widgets/

        ├── app_row.rs
        ├── app_dialog.rs
        └── search.rs
```

---

# Optional Features

## Favorites

Allow users to pin frequently changed settings.

Example:

```
⭐ Browser

⭐ PDF Viewer

⭐ Terminal
```

---

## Change History

Show recent changes:

```
Changed:

PDF Viewer

Before:
Firefox

After:
Evince

[Undo]
```

---

## Restore Defaults

Add:

```
Restore system defaults
```

---

## MIME Inspector

Show:

```
File:

example.pdf


Detected type:

application/pdf


Default handler:

Evince
```

---

# Development Rules

The implementation must:

- use idiomatic Rust
- avoid unnecessary `unwrap()`
- handle errors properly
- separate UI and backend logic
- follow GTK4 best practices
- use libadwaita widgets

Prefer:

- `AdwApplication`
- `AdwApplicationWindow`
- `AdwPreferencesPage`
- `AdwActionRow`
- `AdwNavigationSplitView`
- `AdwToastOverlay`

---

# Final Goal

Create a GNOME-quality application that makes changing default applications simple for beginners while providing advanced MIME management for power users.
