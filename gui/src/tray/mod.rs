/// System tray module
/// 
/// This module handles the system tray icon and menu functionality.
/// Currently uses `tray-icon` and `tao` directly. Dioxus will be integrated
/// for UI components in future tasks.

use tao::event_loop::{ControlFlow, EventLoop};
use tray_icon::{Icon, TrayIconBuilder};

pub mod icon;
pub mod menu;

pub use icon::create_simple_icon;
pub use menu::create_tray_menu;

/// Initializes and runs the system tray application
pub fn run_tray_app() {
    // Create a simple 32x32 icon with a solid color (blue)
    let icon_data = create_simple_icon(32, 32, [0u8, 100u8, 200u8, 255u8]);
    
    let icon = match Icon::from_rgba(icon_data, 32, 32) {
        Ok(icon) => icon,
        Err(e) => {
            eprintln!("Failed to create icon: {}", e);
            return;
        }
    };

    // Create the tray menu
    let menu = create_tray_menu();

    let event_loop = EventLoop::new();
    let _tray_icon = TrayIconBuilder::new()
        .with_icon(icon)
        .with_tooltip("Roro Kube")
        .with_menu(Box::new(menu))
        .build()
        .unwrap();

    // Keep the application running
    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
    });
}

