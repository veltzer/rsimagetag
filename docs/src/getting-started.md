# Getting Started

## First-Time Setup

After building rsimagetag, initialize the tag database:

```bash
rsimagetag db-init
```

This creates `~/.config/rsimagetag/tags.redb`. You only need to do this once.

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
2. **Add tags** to each image — tag people by name and scenes by description.
3. Tags are automatically saved to the database.

## Searching

Use the search functionality to find all photos matching a specific tag. For example:

- Search for "Alice" to find all photos where Alice appears.
- Search for "beach" to find all beach photos.
- Combine tags to narrow results.

## Database

Tags are stored locally as a mapping of image content hashes to tag lists:

```
SHA-256(image) -> ["Alice", "Bob", "beach", "2024"]
```

The database persists between sessions at `~/.config/rsimagetag/tags.redb`. Because it uses content hashes, your tags remain valid even if you rename or move your photo files.

## Commands

```bash
rsimagetag db-init               # Initialize the database (first time only)
rsimagetag db-dump               # Dump the entire database as JSON
rsimagetag tag                    # Browse and tag images in current directory
rsimagetag tag --dir ~/Pictures   # Browse and tag images in a specific directory
rsimagetag version                # Print version and build info
rsimagetag complete bash          # Generate shell completions
```
