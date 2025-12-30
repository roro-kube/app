#![cfg_attr(windows, windows_subsystem = "windows")]

/// GUI Application Layer Entry Point
/// 
/// This is a Dioxus-based GUI application. The current implementation uses
/// `tray-icon` and `tao` directly for the system tray functionality.
/// Dioxus will be integrated for UI components (main window, components, etc.)
/// in future tasks.

mod tray;

// Placeholder modules for future Dioxus components
mod components;
mod icons;
mod pages;
mod layout;

use dioxus::prelude::*;
use components::Greeting;

fn main() {
    // Launch Dioxus desktop application first
    // The tray icon will be initialized after Dioxus starts to avoid conflicts
    // with macOS menu classes that both Dioxus and tray-icon use
    dioxus::launch(App);
}

/// Main Dioxus application component
#[allow(non_snake_case)]
fn App() -> Element {
    // Store tray icon in state to keep it alive
    let mut tray_icon = use_signal(|| None::<tray_icon::TrayIcon>);
    
    use_effect(move || {
        // Initialize tray icon after Dioxus has started
        // This avoids conflicts with macOS menu classes
        match tray::init_tray_icon() {
            Ok(icon) => {
                tray_icon.set(Some(icon));
            }
            Err(e) => {
                eprintln!("Failed to initialize tray icon: {}", e);
            }
        }
    });

    rsx! {
        Greeting {}
    }
}

