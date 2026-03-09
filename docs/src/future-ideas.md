# Future Ideas

This page collects ideas for future development of rsimagetag.

## Face Detection

Integrate face detection to automatically suggest people tags. When a photo is opened, the application could detect faces and either auto-tag known people or prompt the user to identify unknown faces. Libraries like OpenCV (via the `opencv` crate) or ONNX-based models could provide this functionality.

## Tag Categories

Introduce formal tag categories (people, scenes, events, locations, dates) with different UI treatments for each. People tags could show profile thumbnails, scene tags could use icons, and date tags could integrate with a calendar view.

## Bulk Import

Add support for scanning an entire directory tree and presenting untagged images for batch tagging. This would help with initial setup when importing an existing photo collection.

## Export and Sync

Support exporting the tag database in standard formats (JSON, CSV) and syncing it across machines. This would allow sharing tags between devices or backing up the database.

## Smart Albums

Automatically generate albums based on tag combinations. For example, "All beach photos with Alice" or "All 2024 birthday photos". These albums would update dynamically as new photos are tagged.

## Thumbnail Cache

Generate and cache thumbnails for faster browsing of large photo collections. The thumbnails would be stored alongside the tag database and regenerated as needed.
