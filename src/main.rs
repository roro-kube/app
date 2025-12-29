#![cfg_attr(windows, windows_subsystem = "windows")]

use tao::event_loop::{ControlFlow, EventLoop};
use tray_icon::TrayIconBuilder;

fn main() {
    // Create a simple 32x32 icon with a solid color (blue)
    // This creates a minimal icon for the system tray
    let icon_data = create_simple_icon(32, 32, [0u8, 100u8, 200u8, 255u8]); // Blue icon
    
    let icon = match tray_icon::Icon::from_rgba(icon_data, 32, 32) {
        Ok(icon) => icon,
        Err(e) => {
            eprintln!("Failed to create icon: {}", e);
            return;
        }
    };

    let event_loop = EventLoop::new();
    let _tray_icon = TrayIconBuilder::new()
        .with_icon(icon)
        .with_tooltip("Roro Kube")
        .build()
        .unwrap();

    // Keep the application running
    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
    });
}

/// Creates a simple solid color icon as RGBA bytes
fn create_simple_icon(width: u32, height: u32, color: [u8; 4]) -> Vec<u8> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);
    for _ in 0..(width * height) {
        data.extend_from_slice(&color);
    }
    data
}

