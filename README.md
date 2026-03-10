# rsimagetag

A GUI application for tagging and organizing photos, written in Rust.

rsimagetag lets you browse your photo collection, tag people and scenes in each image, and maintains a database mapping image hashes to tag lists. People are referenced by their Google Contacts `resourceName` for unambiguous identification; scene tags are free-form strings. All tags are stored as plain strings in a flat list — the tag type is determined by prefix convention.

## Features

- **Browse photos** in a graphical interface built with egui
- **Tag people** using Google Contacts `resourceName` references — no ambiguity with duplicate names
- **Tag scenes** with free-form labels (e.g., "beach", "wedding", "birthday")
- **Content-based identification** using SHA-256 image hashes — tags survive file renames and moves
- **Persistent ACID database** powered by redb at `~/.config/rsimagetag/tags.redb`
- **Import people** from Google Contacts via rscontacts

## Installation

```bash
cargo install --path .
```

Requires Rust edition 2024.

## Setup

```bash
rsimagetag db-init               # Initialize the database (first time only)
```

## Usage

```bash
rsimagetag tag                    # Browse and tag images in current directory
rsimagetag tag --dir ~/Pictures   # Browse and tag images in a specific directory
rsimagetag db-dump               # Dump the entire database as JSON
rsimagetag version                # Print version and build info
rsimagetag complete bash          # Generate shell completions
```

## How It Works

1. **Browse photos** using the GUI (Prev/Next buttons or keyboard shortcuts).
2. **Tag images** — tags are plain strings stored per image hash:
   - `people/c1234567890` — references a Google Contact (display name looked up from people table)
   - `beach`, `wedding`, `2024` — free-form tags for scenes, events, dates, etc.
3. Tags are stored as `SHA-256(image) -> ["people/c123", "beach", ...]` in the local database.
4. **Search by tag** to find all photos containing a specific person or scene.

Because tags are linked to the image content hash (not the file path), your tags remain valid even if you rename, move, or reorganize your files.

## Building

```bash
cargo build                    # Debug build
cargo build --release          # Release build (stripped, LTO, single codegen unit)
cargo clippy                   # Lint
cargo nextest run              # Run tests
```

## License

See [LICENSE](LICENSE) for details.
