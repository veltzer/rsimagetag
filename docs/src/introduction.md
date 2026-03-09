# rsimagetag - Photo Tagging GUI Tool

A graphical application for tagging and organizing photos, written in Rust.

## Features

- **Browse photos** in a native GUI built with egui
- **Tag people** in each photo (e.g., "Alice", "Bob")
- **Tag scenes** with descriptive labels (e.g., "beach", "wedding", "birthday")
- **Content-based identification** using image hashes — tags survive file renames and moves
- **Persistent database** mapping image hashes to tag lists
- **Search by tag** to find all photos of a specific person or scene

## How It Works

rsimagetag computes a hash of each image's content and stores tags against that hash. This means:

- Renaming a file does not lose its tags.
- Moving a file to a different folder does not lose its tags.
- Duplicate copies of the same image share the same tags.

## Technology

- Built with Rust using [eframe/egui](https://github.com/emilk/egui) for the GUI
- Image hashing for content-based identification
- Local database for tag storage
