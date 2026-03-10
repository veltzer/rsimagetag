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
- **Simple API**: Perfect for key-value mappings.
- **Consistent**: Already used by rscontacts, keeping the tooling consistent across projects.
- **Lightweight**: Minimal overhead compared to SQLite for our use case.

## Tag Schema: Flat String List with Prefix Convention

### Approach Chosen: Tags as Plain Strings

Each image's tags are stored as a flat `Vec<String>`. The type of tag is determined by prefix convention:

- `people/c...` → person reference (Google Contacts `resourceName`)
- anything else → free-form tag (scene, event, location, etc.)

### Alternative Considered: Structured Tags with Separate Fields

An earlier design used a struct with separate `people: Vec<String>` and `scenes: Vec<String>` fields per image. This was rejected in favor of the flat list.

### Why Flat Strings

- **Simpler code**: One `add_tag`/`remove_tag` function instead of two of everything.
- **More flexible**: Adding a new tag type (e.g., `location/...`, `event/...`) requires zero schema changes — just use a new prefix.
- **Simpler JSON**: `["people/c123", "beach"]` instead of `{"people": [...], "scenes": [...]}`.
- **No performance difference**: A typical image has 5-20 tags. The `starts_with("people/c")` check is negligible.
- **The people lookup table handles display**: The people table (`resourceName → display_name`) is a separate lookup table, not part of the tag schema.

## People Identification: Google Contacts resourceName

### Approach Chosen: Reference by `resourceName`

People in photos are identified by their Google Contacts `resourceName` (e.g., `people/c1234567890`), not by display name.

### Alternative Considered: Display Name Strings

Using display names like `"Alice Smith"` would be simpler but creates problems with duplicate names and name changes.

### Why resourceName

- **Unique**: Google guarantees each contact has a unique `resourceName`. Two contacts named "Mike" have different resourceNames.
- **Stable**: The `resourceName` does not change when a contact's name is updated. If "Bob Smith" changes to "Robert Smith", all photo tags still reference the same person.
- **Scalable**: Works equally well with 10 contacts or 10,000.
- **Integration**: Importing from rscontacts (which uses the Google People API) naturally provides resourceNames.

The display name is stored separately in the people lookup table and resolved at display time.
