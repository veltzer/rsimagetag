# Design Decisions

## GUI Framework: egui via eframe

### Approach Chosen: egui/eframe

rsimagetag uses [egui](https://github.com/emilk/egui) as its GUI framework, running via the eframe integration.

### Alternatives Considered

- **gtk4-rs**: Native Linux look and feel, but adds a system dependency on GTK4 libraries.
- **Qt via cxx-qt**: Mature and feature-rich, but the Rust bindings add significant complexity.
- **iced**: Elm-inspired architecture, but less mature ecosystem than egui.
- **Tauri**: Web-based UI, but adds a web stack dependency for a desktop app.

### Why egui

- **Pure Rust**: No system dependencies beyond OpenGL/Vulkan — builds anywhere Rust builds.
- **Simple API**: Immediate mode GUI is easy to prototype and iterate on.
- **Good image support**: egui handles image rendering well, which is critical for a photo app.
- **Active community**: Well-maintained with frequent releases and good documentation.

## Image Identification: Content Hashing

### Approach Chosen: SHA-256 Hash of File Content

Each image is identified by a SHA-256 hash of its file content. Tags are stored against this hash in the database.

### Alternative Considered: File Path

Using the file path as the identifier would be simpler but fragile — renaming or moving a file would lose all its tags.

### Why Content Hashing

- **Robust**: Tags survive file renames, moves, and reorganization.
- **Deduplication**: Duplicate copies of the same image automatically share tags.
- **Trade-off**: Slightly slower on first load (must read and hash each file), but hashes can be cached.

## Tag Database: redb

### Approach Chosen: redb Embedded Database

rsimagetag uses [redb](https://github.com/cberner/redb) as its embedded key-value store, located at `~/.config/rsimagetag/tags.redb`.

### Alternatives Considered

- **sled**: Pure Rust embedded database, but maintenance has been inconsistent and its future is uncertain.
- **SQLite via rusqlite**: Battle-tested relational database, but heavier than needed for a simple key-value mapping. Adds a C dependency.
- **serde_json flat file**: Simplest possible approach, but no ACID guarantees — a crash during write could corrupt the entire database.

### Why redb

- **Pure Rust**: No system dependencies or C libraries to link.
- **ACID-compliant**: Crash-safe — tags are never lost due to power failures or unexpected termination.
- **Simple API**: Perfect for a key-value mapping of `hash -> tags`.
- **Consistent**: Already used by rscontacts, keeping the tooling consistent across projects.
- **Lightweight**: Minimal overhead compared to SQLite for our use case.

### Schema

The database has a single table (`tags`) with:

- **Key**: SHA-256 hex string (64 characters)
- **Value**: JSON-encoded array of tag strings (e.g., `["Alice","Bob","beach"]`)

JSON encoding for the value keeps the schema simple while allowing variable-length tag lists without a secondary index or join table.
