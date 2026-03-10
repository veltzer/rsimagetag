/// Generate a 64x64 RGBA icon programmatically.
/// Design: teal photo frame with an orange tag in the bottom-right corner.
pub fn generate_icon() -> (Vec<u8>, u32, u32) {
    const SIZE: u32 = 64;
    let mut pixels = vec![0u8; (SIZE * SIZE * 4) as usize];

    let set = |pixels: &mut Vec<u8>, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8| {
        if x < SIZE && y < SIZE {
            let i = ((y * SIZE + x) * 4) as usize;
            pixels[i] = r;
            pixels[i + 1] = g;
            pixels[i + 2] = b;
            pixels[i + 3] = a;
        }
    };

    let fill_rect = |pixels: &mut Vec<u8>, x0: u32, y0: u32, x1: u32, y1: u32, r: u8, g: u8, b: u8| {
        for y in y0..y1.min(SIZE) {
            for x in x0..x1.min(SIZE) {
                set(pixels, x, y, r, g, b, 255);
            }
        }
    };

    let fill_circle = |pixels: &mut Vec<u8>, cx: i32, cy: i32, radius: i32, r: u8, g: u8, b: u8| {
        for y in (cy - radius)..=(cy + radius) {
            for x in (cx - radius)..=(cx + radius) {
                let dx = x - cx;
                let dy = y - cy;
                if dx * dx + dy * dy <= radius * radius {
                    if x >= 0 && x < SIZE as i32 && y >= 0 && y < SIZE as i32 {
                        set(pixels, x as u32, y as u32, r, g, b, 255);
                    }
                }
            }
        }
    };

    // Background: rounded teal rectangle (photo frame)
    let teal = (0x00u8, 0x96u8, 0x88u8); // teal-600
    fill_rect(&mut pixels, 4, 4, 60, 60, teal.0, teal.1, teal.2);

    // Inner white area (photo)
    fill_rect(&mut pixels, 8, 8, 56, 56, 0xF5, 0xF5, 0xF5);

    // Simple mountain landscape in the photo area
    // Sky: light blue
    fill_rect(&mut pixels, 9, 9, 55, 38, 0x87, 0xCE, 0xEB);

    // Sun: yellow circle
    fill_circle(&mut pixels, 46, 18, 5, 0xFF, 0xD5, 0x4F);

    // Mountain 1: dark green triangle (left)
    for y in 24..48 {
        let half_width = (y - 24) * 20 / 24;
        let cx = 22i32;
        for x in (cx - half_width)..=(cx + half_width) {
            if x >= 9 && x < 55 {
                set(&mut pixels, x as u32, y as u32, 0x2E, 0x7D, 0x32, 255);
            }
        }
    }

    // Mountain 2: lighter green triangle (right, overlapping)
    for y in 30..48 {
        let half_width = (y - 30) * 16 / 18;
        let cx = 40i32;
        for x in (cx - half_width)..=(cx + half_width) {
            if x >= 9 && x < 55 {
                set(&mut pixels, x as u32, y as u32, 0x43, 0xA0, 0x47, 255);
            }
        }
    }

    // Ground: green
    fill_rect(&mut pixels, 9, 44, 55, 55, 0x4C, 0xAF, 0x50);

    // Tag label: orange rectangle in bottom-right corner
    let orange = (0xFF, 0x98, 0x00); // orange-500
    fill_rect(&mut pixels, 38, 42, 60, 56, orange.0, orange.1, orange.2);

    // Tag hole: small dark circle
    fill_circle(&mut pixels, 41, 46, 2, 0x60, 0x60, 0x60);

    // Tag text: white "T" on the tag
    // Horizontal bar of T
    fill_rect(&mut pixels, 46, 44, 56, 46, 0xFF, 0xFF, 0xFF);
    // Vertical bar of T
    fill_rect(&mut pixels, 49, 46, 52, 54, 0xFF, 0xFF, 0xFF);

    (pixels, SIZE, SIZE)
}
