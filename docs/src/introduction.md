# rsimagetag - Photo Tagging GUI Tool

A graphical application for tagging and organizing photos, written in Rust.

## Features

- **Browse photos** in a native GUI built with egui
- **Tag people** using Google Contacts `resourceName` references — no ambiguity with duplicate names
- **Tag scenes** with free-form labels (e.g., "beach", "wedding", "birthday")
- **Content-based identification** using SHA-256 image hashes — tags survive file renames and moves
- **Persistent ACID database** powered by redb at `~/.config/rsimagetag/tags.redb`
- **Import people** from Google Contacts via rscontacts
- **Search by tag** to find all photos of a specific person or scene

## How It Works

rsimagetag computes a SHA-256 hash of each image's content and stores tags against that hash. Tags are plain strings — if a tag starts with `people/c` it references a Google Contact, otherwise it is a free-form tag (scene, event, location, etc.).

- Renaming a file does not lose its tags.
- Moving a file to a different folder does not lose its tags.
- Duplicate copies of the same image share the same tags.

## Technology

- Built with Rust using [eframe/egui](https://github.com/emilk/egui) for the GUI
- [redb](https://github.com/cberner/redb) embedded database for ACID-safe tag storage
- SHA-256 ([sha2](https://crates.io/crates/sha2)) for content-based image identification
- [clap](https://crates.io/crates/clap) for CLI
- Integration with [rscontacts](https://github.com/veltzer/rscontacts) for people import
