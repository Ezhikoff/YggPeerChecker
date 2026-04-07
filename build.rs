//! Build script — embeds application icon on Windows
use std::io::Write;

#[cfg(windows)]
fn main() {
    // Generate ICO file with multiple sizes and embed it
    let icon_dir = generate_ico();
    std::fs::write("app.ico", &icon_dir).expect("Failed to write app.ico");

    let mut res = winresource::WindowsResource::new();
    res.set_icon("app.ico");
    res.set("LegalCopyright", "MIT License");
    res.set("FileDescription", "YggPeerChecker - Yggdrasil Peer Checker");
    res.set("ProductName", "YggPeerChecker");
    res.compile().expect("Failed to compile resources");
}

#[cfg(not(windows))]
fn main() {
    // On non-Windows, just generate the ICO file for reference
    let icon_data = generate_ico();
    std::fs::write("app.ico", &icon_data).expect("Failed to write app.ico");
    println!("cargo:rerun-if-changed=build.rs");
}

fn generate_ico() -> Vec<u8> {
    let mut buf = Vec::new();

    // ICONDIR: reserved(2)=0, type(2)=1, count(2)=4
    buf.write_all(&[0u8, 0, 1, 0, 4, 0]).unwrap();

    let sizes: &[u32] = &[16, 32, 48, 256];
    let mut offsets: Vec<u32> = Vec::new();
    let mut images: Vec<Vec<u8>> = Vec::new();

    for &size in sizes {
        let img = generate_bmp_image(size);
        offsets.push(img.len() as u32);
        images.push(img);
    }

    // Fix offsets to be absolute
    let mut offset = 6 + 16 * sizes.len() as u32;
    for i in 0..sizes.len() {
        let abs = offset;
        offsets[i] = abs;
        offset += images[i].len() as u32;
    }

    // ICONDIRENTRY for each size
    for (i, &size) in sizes.iter().enumerate() {
        buf.write_all(&[
            if size < 256 { size as u8 } else { 0 }, // width
            if size < 256 { size as u8 } else { 0 }, // height
            0,   // color palette
            0,   // reserved
            1, 0, // color planes = 1
            32, 0, // bits per pixel = 32
        ]).unwrap();
        buf.write_all(&(images[i].len() as u32).to_le_bytes()).unwrap();
        buf.write_all(&offsets[i].to_le_bytes()).unwrap();
    }

    // Image data
    for img in &images {
        buf.write_all(img).unwrap();
    }

    buf
}

fn generate_bmp_image(size: u32) -> Vec<u8> {
    let s = size as i32;
    let padding = (size / 8) as i32;
    let r = (size / 4) as i32;
    let cx = s / 2;
    let cy = s / 2;

    let nodes: &[(i32, i32)] = &[
        (cx, cy - r),
        (cx - r, cy + r / 2),
        (cx + r, cy + r / 2),
        (cx - r / 2, cy - r / 3),
        (cx + r / 2, cy + r / 3),
    ];
    let node_r = (size as i32 / 16).max(2);

    // BMP data: 32-bit BGRA, bottom-up, XOR mask + AND mask
    let height = size * 2; // XOR + AND

    // We need to store pixels, then output bottom-up
    let mut pixels = vec![(0u8, 0u8, 0u8, 0u8); (size * size) as usize];

    for y in 0..size as i32 {
        for x in 0..size as i32 {
            let dx = x - cx;
            let dy = y - cy;
            let dist_sq = dx * dx + dy * dy;
            let max_r = s / 2 - padding - 1;
            if dist_sq <= max_r * max_r {
                let mut is_node = false;
                for &(nx, ny) in nodes {
                    let ndx = x - nx;
                    let ndy = y - ny;
                    if ndx * ndx + ndy * ndy <= node_r * node_r {
                        is_node = true;
                        break;
                    }
                }
                let idx = (y * s as i32 + x) as usize;
                if is_node {
                    pixels[idx] = (255, 255, 255, 255); // White
                } else {
                    pixels[idx] = (0, 180, 60, 255); // Green (BGRA)
                }
            }
        }
    }

    // Draw connecting lines between nodes (Bresenham)
    for i in 0..nodes.len() {
        let (x1, y1) = nodes[i];
        let (x2, y2) = nodes[(i + 1) % nodes.len()];
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;
        let (mut mx, mut my) = (x1, y1);
        loop {
            if mx >= 0 && mx < s as i32 && my >= 0 && my < s as i32 {
                let idx = (my * s as i32 + mx) as usize;
                pixels[idx] = (255, 255, 255, 255);
            }
            if mx == x2 && my == y2 {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                mx += sx;
            }
            if e2 < dx {
                err += dx;
                my += sy;
            }
        }
    }

    // BMP header (40 bytes)
    let mut bmp = Vec::new();
    let row_size = size * 4;
    let image_size = row_size * height;
    bmp.write_all(&[
        40, 0, 0, 0, // header size
    ]).unwrap();
    bmp.write_all(&size.to_le_bytes()).unwrap(); // width
    bmp.write_all(&height.to_le_bytes()).unwrap(); // height
    bmp.write_all(&1u16.to_le_bytes()).unwrap(); // planes
    bmp.write_all(&32u16.to_le_bytes()).unwrap(); // bpp
    bmp.write_all(&0u32.to_le_bytes()).unwrap(); // compression
    bmp.write_all(&image_size.to_le_bytes()).unwrap(); // image size
    bmp.write_all(&0i32.to_le_bytes()).unwrap(); // x ppm
    bmp.write_all(&0i32.to_le_bytes()).unwrap(); // y ppm
    bmp.write_all(&0u32.to_le_bytes()).unwrap(); // colors used
    bmp.write_all(&0u32.to_le_bytes()).unwrap(); // important colors

    // XOR mask: bottom-up rows
    for y in (0..size as i32).rev() {
        for x in 0..size as i32 {
            let (b, g, r, a) = pixels[(y * s as i32 + x) as usize];
            bmp.write_all(&[b, g, r, a]).unwrap();
        }
    }

    // AND mask: all zeros (fully opaque)
    for _ in 0..(size * size) {
        bmp.write_all(&[0u8]).unwrap();
    }

    bmp
}
