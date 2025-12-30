/// Icon loading utilities for the system tray
/// 
/// Loads PNG icons from assets and converts them to RGBA format for the tray icon.

use dioxus::desktop::trayicon::Icon;
use image::load_from_memory;

/// Loads the tray icon from embedded PNG assets
/// 
/// Returns a 32x32 icon by default, or 64x64 for high-DPI displays.
/// Falls back to a simple solid-color icon if the PNG fails to load.
pub fn load_tray_icon() -> Result<Icon, String> {
    // Try to load 32x32 icon first (standard size)
    if let Ok(icon) = load_tray_icon_size(32) {
        return Ok(icon);
    }

    // Fallback to 64x64 if available
    if let Ok(icon) = load_tray_icon_size(64) {
        return Ok(icon);
    }

    // Final fallback: generate a simple solid-color icon
    create_fallback_icon()
}

/// Loads a specific size tray icon from embedded PNG
pub fn load_tray_icon_size(size: u32) -> Result<Icon, String> {
    // Load PNG from embedded assets
    let png_bytes: &[u8] = match size {
        32 => include_bytes!("../../assets/branding/logo-32.png").as_slice(),
        64 => include_bytes!("../../assets/branding/logo-64.png").as_slice(),
        _ => return Err(format!("Unsupported icon size: {}", size)),
    };

    // Decode PNG to RGBA
    let img = load_from_memory(png_bytes)
        .map_err(|e| format!("Failed to decode PNG: {}", e))?;

    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let rgba_bytes = rgba.into_raw();

    // Create icon from RGBA bytes
    Icon::from_rgba(rgba_bytes, width, height)
        .map_err(|e| format!("Failed to create icon from RGBA: {}", e))
}

/// Creates a simple fallback icon with a solid color
/// 
/// Used when PNG loading fails.
fn create_fallback_icon() -> Result<Icon, String> {
    // Create a simple 32x32 icon with a solid color (blue)
    let icon_data = create_simple_icon(32, 32, [0u8, 100u8, 200u8, 255u8]);
    Icon::from_rgba(icon_data, 32, 32)
        .map_err(|e| format!("Failed to create fallback icon: {}", e))
}

/// Creates a simple solid color icon as RGBA bytes
/// 
/// This is a fallback function used when PNG loading fails.
fn create_simple_icon(width: u32, height: u32, color: [u8; 4]) -> Vec<u8> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);
    for _ in 0..(width * height) {
        data.extend_from_slice(&color);
    }
    data
}
