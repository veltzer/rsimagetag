# Future Ideas

This page collects ideas for future development of rsimagetag.

## Face Detection

Integrate face detection to automatically suggest people tags. When a photo is opened, the application could detect faces and either auto-tag known people or prompt the user to identify unknown faces. Libraries like OpenCV (via the `opencv` crate) or ONNX-based models could provide this functionality.

## Additional Tag Prefixes

The flat string tag system is designed for extensibility via prefixes. Future tag types could include:

- `location/...` — geographic locations (e.g., `location/paris`, `location/home`)
- `event/...` — named events (e.g., `event/wedding-2024`, `event/birthday-bob`)
- `date/...` — date tags (e.g., `date/2024-07`)

No schema changes would be needed — just new prefix conventions and UI support.

## Bulk Import

Add support for scanning an entire directory tree and presenting untagged images for batch tagging. This would help with initial setup when importing an existing photo collection.

## Export and Sync

Support exporting the tag database in standard formats (JSON, CSV) and syncing it across machines. The `db-dump` command already provides JSON export; a corresponding `db-import` command would enable restoring from backups or syncing between devices.

## Smart Albums

Automatically generate albums based on tag combinations. For example, "All beach photos with Alice" or "All 2024 birthday photos". These albums would update dynamically as new photos are tagged.

## Thumbnail Cache

Generate and cache thumbnails for faster browsing of large photo collections. The thumbnails would be stored alongside the tag database and regenerated as needed.

## rscontacts JSON Export

Add a `--json` flag to rscontacts' `list` command to output contacts as structured JSON with `resourceName` and display name fields. This would make the import into rsimagetag more robust than parsing the current text output.
