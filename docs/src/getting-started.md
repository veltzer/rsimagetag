# Getting Started

## Launching the Application

After building, run:

```bash
rsimagetag
```

This opens the GUI window.

## Basic Workflow

1. **Open a photo or folder** of photos using the file browser.
2. **Browse** through your images in the main view.
3. **Add tags** to each image — tag people by name and scenes by description.
4. Tags are automatically saved to the local database.

## Searching

Use the search functionality to find all photos matching a specific tag. For example:

- Search for "Alice" to find all photos where Alice appears.
- Search for "beach" to find all beach photos.
- Combine tags to narrow results.

## Database

Tags are stored locally as a mapping of image content hashes to tag lists:

```
hash(image) -> ["Alice", "Bob", "beach", "2024"]
```

The database persists between sessions. Because it uses content hashes, your tags remain valid even if you rename or move your photo files.
