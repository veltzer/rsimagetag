# Getting Started

## First-Time Setup

After building rsimagetag, initialize the tag database:

```bash
rsimagetag db-init
```

This creates `~/.config/rsimagetag/tags.redb` with the tags and people lookup tables. You only need to do this once.

## Importing People from Google Contacts

Import your contacts so you can tag people in photos by name:

```bash
rsimagetag db-import-rscontacts
```

This requires [rscontacts](https://github.com/veltzer/rscontacts) to be installed and authenticated. The import populates the people lookup table with `resourceName → display_name` entries.

## Launching the Application

Browse and tag images in the current directory:

```bash
rsimagetag tag
```

Or specify a directory:

```bash
rsimagetag tag --dir ~/Pictures
```

## Basic Workflow

1. **Navigate** through images using the Prev/Next buttons or keyboard shortcuts (Arrow keys, N/P).
2. **Add tags** to each image — select people from the imported contacts list, or type scene tags.
3. Tags are automatically saved to the database.

## How Tags Work

Tags are plain strings stored in a flat list per image:

```json
["people/c1234567890", "people/c9876543210", "beach", "sunset"]
```

- Tags starting with `people/c` reference a Google Contact — the UI shows the display name from the people lookup table.
- All other tags are free-form (scenes, events, locations, etc.).

## Inspecting the Database

Dump the full database as JSON:

```bash
rsimagetag db-dump
```

Output:

```json
{
  "people": {
    "people/c1234567890": "Alice Smith",
    "people/c9876543210": "Bob Jones"
  },
  "image_tags": {
    "a1b2c3...": ["people/c1234567890", "beach"],
    "d4e5f6...": ["people/c9876543210", "sunset"]
  }
}
```

## Commands

```bash
rsimagetag db-init               # Initialize the database (first time only)
rsimagetag db-dump               # Dump the entire database as JSON
rsimagetag tag                    # Browse and tag images in current directory
rsimagetag tag --dir ~/Pictures   # Browse and tag images in a specific directory
rsimagetag version                # Print version and build info
rsimagetag complete bash          # Generate shell completions
```
