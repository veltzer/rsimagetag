# Concepts

## Image Hashing

rsimagetag identifies images by their content hash rather than their file path. When you open an image, the application computes a SHA-256 hash of the file data. This hash serves as the unique identifier for that image in the database.

Benefits:

- **Rename-proof**: Moving or renaming a file does not affect its tags.
- **Deduplication**: Multiple copies of the same image automatically share tags.
- **Portable**: The database is meaningful even if your folder structure changes.

## Tags

Tags are **plain strings** stored in a flat list per image. The type of a tag is determined by convention:

### People Tags

A tag that starts with `people/c` is a reference to a Google Contacts `resourceName` (e.g., `people/c1234567890`). This is the stable unique identifier that Google assigns to each contact.

Benefits of using `resourceName` instead of a display name:

- **No ambiguity**: Two contacts named "Mike" have different resourceNames.
- **Rename-safe**: If someone changes their name in Google Contacts, the tag still points to the right person — only the display name in the people lookup table needs updating.
- **Scalable**: Works with 10 or 10,000 contacts.

### Scene / Free-Form Tags

Any tag that does **not** start with `people/c` is a free-form tag. Use these for scenes, events, locations, dates, or anything else. Examples: `beach`, `wedding`, `birthday`, `vacation`, `2024`.

### Tag Detection

The application determines the tag type with a simple prefix check:

```
if tag.starts_with("people/c") → person reference → look up display name
else                           → free-form tag    → display as-is
```

This convention is extensible — future tag types can use new prefixes (e.g., `location/...`, `event/...`) without schema changes.

## Database

The database is an embedded [redb](https://github.com/cberner/redb) store located at `~/.config/rsimagetag/tags.redb`. It contains two tables:

### Tags Table

Maps image content hashes to tag lists.

| Key | Value |
|-----|-------|
| SHA-256 hex (64 chars) | JSON array of tag strings |

Example:

```json
["people/c1234567890", "people/c9876543210", "beach", "sunset"]
```

### People Lookup Table

Maps Google Contacts `resourceName` to display name. This is a lookup-only table used by the UI to show human-readable names instead of opaque IDs.

| Key | Value |
|-----|-------|
| `people/c1234567890` | `Alice Smith` |
| `people/c9876543210` | `Bob Jones` |

This table is populated by importing contacts from rscontacts.

### Initialization

The database must be initialized before first use:

```bash
rsimagetag db-init
```

This creates the `~/.config/rsimagetag/` directory and the `tags.redb` database file with both tables.

The database is ACID-compliant — tags are never lost due to crashes or power failures.
