/// Icon creation utilities for the system tray

/// Creates a simple solid color icon as RGBA bytes
pub fn create_simple_icon(width: u32, height: u32, color: [u8; 4]) -> Vec<u8> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);
    for _ in 0..(width * height) {
        data.extend_from_slice(&color);
    }
    data
}

