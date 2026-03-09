# Concepts

## Image Hashing

rsimagetag identifies images by their content hash rather than their file path. When you open an image, the application computes a cryptographic hash of the image data. This hash serves as the unique identifier for that image in the database.

Benefits:

- **Rename-proof**: Moving or renaming a file does not affect its tags.
- **Deduplication**: Multiple copies of the same image automatically share tags.
- **Portable**: The database is meaningful even if your folder structure changes.

## Tags

Tags are free-form text labels attached to an image. There are two main categories:

### People Tags

Tags identifying people in the photo. Examples: "Alice", "Bob", "Mom", "Dad".

### Scene Tags

Tags describing the content or context of the photo. Examples: "beach", "wedding", "birthday", "vacation", "sunset".

There is no enforced distinction between people and scene tags — they are all stored in the same tag list. The separation is purely conceptual to help you organize.

## Database

The database is a simple key-value store:

- **Key**: The content hash of the image (e.g., SHA-256)
- **Value**: A list of tags associated with that image

The database is stored locally and persists between sessions.
