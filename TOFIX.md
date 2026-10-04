# TOFIX

Findings from a code scan on 2026-10-04.

## High

- `docs/src/getting-started.md:18` - documents `rsimagetag db-import-rscontacts`, a subcommand that does not exist; the CLI has `db-import-people <file>` (`src/cli.rs:27`), which reads a JSON file from `rscontacts export-json --short`. Fix the command and explain the export step.
- `README.md:5` - README (lines 5, 10-11, 43-47) and `docs/src/getting-started.md:40` promise tagging people and scenes in the GUI and searching by tag, but the GUI (`src/lib.rs:129-216`) only does prev/next/trash: it never opens the database or calls `add_tag`/`get_tags`/`hash_file` (`src/db.rs:64,149,179`), and there is no search command. Either implement tagging/search in the UI or document the current state and move the rest to `docs/src/future-ideas.md`.

## Medium

- `src/lib.rs:108` - `trash_current` moves to `~/Trash/rsimagetag/<file_name>` with `std::fs::rename`, which silently replaces an existing file of the same name, so trashing two `IMG_0001.jpg` from different directories (the scan is recursive, `src/lib.rs:13`) destroys the first one. Pick a unique destination name (or use the freedesktop trash) before renaming.
- `src/lib.rs:109` - `std::fs::rename` fails with `EXDEV` when the image lives on a different filesystem from `$HOME` (external drive, NAS mount), so Trash does not work there. Fall back to copy+remove on a cross-device error.

## Low

- `src/lib.rs:178` - the status bar says "No images found in current directory." (and `src/lib.rs:207` says "Run rsimagetag from a directory containing images.") even when `tag --dir` pointed at another directory; show the scanned directory instead.
- `src/icon.rs:85` - doc comment says `install_file` "Returns a status message", but it returns `Result<()>` and prints instead; fix the comment.
- `src/icon.rs:127` - comment says "then update icon cache", but no icon cache update (e.g. `gtk-update-icon-cache`) is run; either run it or drop the claim.
- `src/cli.rs:45` - `parse_shell` is public but never used anywhere (clap parses `Shell` via `value_enum` at line 35); remove it.
- `docs/src/getting-started.md:79` - the commands list (lines 79-85) and `README.md:32-37` omit the existing `db-import-people` and `install-desktop` subcommands (`src/cli.rs:28,39`); list them.
