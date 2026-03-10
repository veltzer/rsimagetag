/// Generate a 64x64 RGBA icon programmatically.
/// Design: white "T" on a teal rounded-rectangle background.
pub fn generate_icon() -> (Vec<u8>, u32, u32) {
    const SIZE: u32 = 64;
    let mut pixels = vec![0u8; (SIZE * SIZE * 4) as usize];

    let set = |pixels: &mut Vec<u8>, x: u32, y: u32, r: u8, g: u8, b: u8| {
        if x < SIZE && y < SIZE {
            let i = ((y * SIZE + x) * 4) as usize;
            pixels[i] = r;
            pixels[i + 1] = g;
            pixels[i + 2] = b;
            pixels[i + 3] = 255;
        }
    };

    let fill_rect = |pixels: &mut Vec<u8>, x0: u32, y0: u32, x1: u32, y1: u32, r: u8, g: u8, b: u8| {
        for y in y0..y1.min(SIZE) {
            for x in x0..x1.min(SIZE) {
                set(pixels, x, y, r, g, b);
            }
        }
    };

    // Background: teal rounded rectangle
    let (tr, tg, tb) = (0x00u8, 0x96u8, 0x88u8); // teal-600
    // Fill main body
    fill_rect(&mut pixels, 6, 4, 58, 60, tr, tg, tb);
    fill_rect(&mut pixels, 4, 6, 60, 58, tr, tg, tb);
    // Round corners with 2px radius
    for &(cx, cy) in &[(6, 6), (57, 6), (6, 57), (57, 57)] {
        for dy in -2i32..=2 {
            for dx in -2i32..=2 {
                if dx * dx + dy * dy <= 5 {
                    let px = (cx as i32 + dx) as u32;
                    let py = (cy as i32 + dy) as u32;
                    set(&mut pixels, px, py, tr, tg, tb);
                }
            }
        }
    }

    // White "T" centered in the icon
    // Horizontal bar: y 12..20, x 12..52
    fill_rect(&mut pixels, 12, 12, 52, 20, 0xFF, 0xFF, 0xFF);
    // Vertical bar: y 20..52, x 26..38
    fill_rect(&mut pixels, 26, 20, 38, 52, 0xFF, 0xFF, 0xFF);

    (pixels, SIZE, SIZE)
}

/// Generate the icon as PNG bytes in memory.
fn generate_icon_png() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let (pixels, w, h) = generate_icon();
    let img = image::RgbaImage::from_raw(w, h, pixels)
        .ok_or("failed to create image from icon pixels")?;
    let mut buf = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageFormat::Png)?;
    Ok(buf.into_inner())
}

/// Generate the .desktop file contents.
fn generate_desktop_contents() -> Result<String, Box<dyn std::error::Error>> {
    let exe = std::env::current_exe()?;
    Ok(format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=rsimagetag\n\
         Comment=Photo tagging and organization tool\n\
         Exec={} tag\n\
         Icon=rsimagetag\n\
         Terminal=false\n\
         Categories=Graphics;Photography;\n\
         StartupWMClass=rsimagetag\n",
        exe.display()
    ))
}

/// Compare new content against an existing file and install if needed.
/// Returns a status message describing what happened.
fn install_file(path: &std::path::Path, new_contents: &[u8], label: &str) -> Result<(), Box<dyn std::error::Error>> {
    if path.exists() {
        let existing = std::fs::read(path)?;
        if existing == new_contents {
            println!("{label}: already up to date at {}", path.display());
        } else {
            std::fs::write(path, new_contents)?;
            println!("{label}: updated at {}", path.display());
        }
    } else {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, new_contents)?;
        println!("{label}: installed to {}", path.display());
    }
    Ok(())
}

/// Install a .desktop file and icon for KDE/GNOME taskbar integration.
/// - Icon: ~/.local/share/icons/hicolor/64x64/apps/rsimagetag.png
/// - Desktop: ~/.local/share/applications/rsimagetag.desktop
pub fn install_desktop() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = dirs::data_dir().ok_or("could not determine data directory")?;

    let icon_png = generate_icon_png()?;
    let icon_path = data_dir.join("icons/hicolor/64x64/apps/rsimagetag.png");
    install_file(&icon_path, &icon_png, "Icon")?;

    let desktop_contents = generate_desktop_contents()?;
    let desktop_path = data_dir.join("applications/rsimagetag.desktop");
    install_file(&desktop_path, desktop_contents.as_bytes(), "Desktop file")?;

    Ok(())
}
