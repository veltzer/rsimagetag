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

### Approach Chosen: Hash of Image Content

Each image is identified by a cryptographic hash (e.g., SHA-256) of its file content. Tags are stored against this hash.

### Alternative Considered: File Path

Using the file path as the identifier would be simpler but fragile — renaming or moving a file would lose all its tags.

### Why Content Hashing

- **Robust**: Tags survive file renames, moves, and reorganization.
- **Deduplication**: Duplicate copies of the same image automatically share tags.
- **Trade-off**: Slightly slower on first load (must read and hash each file), but hashes can be cached.
