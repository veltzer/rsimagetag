# Desktop Integration

rsimagetag provides a built-in command to install itself as a proper desktop application
with icon and `.desktop` file support for KDE, GNOME, and other freedesktop-compliant
desktop environments.

## Installing the Desktop Entry

After building, run:

```bash
rsimagetag install-desktop
```

This installs:

- **Icon files** at `~/.local/share/icons/hicolor/{64x64,128x128,256x256}/apps/rsimagetag.png`
- **Desktop entry** at `~/.local/share/applications/rsimagetag.desktop`
- **Icon theme index** — copies `/usr/share/icons/hicolor/index.theme` into
  `~/.local/share/icons/hicolor/` if it is missing

After installation, you may need to update your icon cache:

```bash
gtk-update-icon-cache ~/.local/share/icons/hicolor/
kbuildsycoca6   # KDE only
```

## How the Icon Works

rsimagetag generates its application icon programmatically at runtime — there are no
static icon image files in the repository. The icon is a white "T" on a teal (#009688)
rounded-rectangle background.

- **Window icon**: Generated via `generate_icon()` and passed to eframe's
  `ViewportBuilder::with_icon()` at startup. This sets the icon shown in the window
  title bar.
- **Taskbar/launcher icon**: Generated as PNG files by `install-desktop` and installed
  into the hicolor icon theme at multiple sizes (64, 128, 256 pixels). The `.desktop`
  file references these via `Icon=rsimagetag`.

## Troubleshooting

### Icon not showing in taskbar or application launcher

This is typically caused by one or more of the following issues:

#### Missing icon theme index

The `~/.local/share/icons/hicolor/` directory must contain an `index.theme` file for
the icon cache to be built correctly. Without it, `gtk-update-icon-cache` fails and
desktop environments cannot discover installed icons.

Run `install-desktop` again — it now copies the system `index.theme` automatically
if it is missing. Alternatively, copy it manually:

```bash
cp /usr/share/icons/hicolor/index.theme ~/.local/share/icons/hicolor/index.theme
```

#### Stale icon cache

After installing or updating icons, the GTK icon cache and KDE's sycoca database
must be refreshed:

```bash
gtk-update-icon-cache ~/.local/share/icons/hicolor/
kbuildsycoca6   # KDE Plasma
```

If the icon still does not appear, try logging out and back in.

#### Icon size too small

Some desktop environments (especially KDE Plasma) request icons at sizes larger than
64x64 for the taskbar and application launcher. If only a 64x64 icon is available,
the desktop environment may fall back to a generic icon instead of upscaling.

The `install-desktop` command installs icons at 64x64, 128x128, and 256x256 to cover
all common sizes.

#### StartupWMClass mismatch

The `.desktop` file must have a `StartupWMClass` that matches the window's
`resourceClass` as reported by the desktop environment. For rsimagetag, both are set
to `rsimagetag`. The `app_id` passed to eframe's `ViewportBuilder::with_app_id()`
controls this value on Wayland.

You can verify the window class using KDE's window properties (right-click title bar
and select "More Actions" > "Configure Special Window Settings") or by running:

```bash
kdotool search --class rsimagetag
```

#### Wayland vs X11

On Wayland, the application ID (set via `with_app_id("rsimagetag")`) is used to match
the window to its `.desktop` file. On X11, the `WM_CLASS` property is used instead.
Ensure the `app_id` / `WM_CLASS` matches the `StartupWMClass` in the `.desktop` file.
