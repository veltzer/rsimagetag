# rsimagetag

A GUI application for tagging and organizing photos, written in Rust.

rsimagetag lets you browse your photo collection, tag people and scenes in each image, and maintains a database mapping image hashes to tag lists. This makes it easy to search and organize photos by who is in them or what they depict, regardless of file names or folder structure.

## Features

- **Browse photos** in a graphical interface built with egui
- **Tag people** in each photo
- **Tag scenes** with descriptive labels (e.g., "beach", "wedding", "birthday")
- **Content-based identification** using image hashes — tags survive file renames and moves
- **Persistent database** mapping image hashes to their tag lists

## Installation

```bash
cargo install --path .
```

Requires Rust edition 2024.

## Usage

Launch the application:

```bash
rsimagetag
```

## How It Works

1. **Open a photo** or a folder of photos.
2. **Tag people and scenes** using the GUI.
3. Tags are stored in a local database as `hash(image) -> [tag1, tag2, ...]`.
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
