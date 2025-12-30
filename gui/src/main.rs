#![cfg_attr(windows, windows_subsystem = "windows")]

/// GUI Application Layer Entry Point
/// 
/// This is a Dioxus-based GUI application with system tray support.
/// The tray icon uses Dioxus Desktop's native trayicon module for seamless
/// integration and cross-platform compatibility.

mod tray;

// Placeholder modules for future Dioxus components
mod components;
mod icons;
mod pages;
mod layout;

use dioxus::prelude::*;
use components::Greeting;

fn main() {
    // Launch Dioxus desktop application
    // The tray icon will be initialized in the App component after Dioxus starts
    dioxus::launch(App);
}

/// Main Dioxus application component
#[allow(non_snake_case)]
fn App() -> Element {
    // Store tray manager in state to keep it alive
    let mut tray_manager = use_signal(|| None::<tray::TrayManager>);
    
    // Initialize tray icon after component mounts
    use_effect(move || {
        match tray::init_tray() {
            Ok(manager) => {
                tray_manager.set(Some(manager));
                println!("Tray icon initialized successfully");
            }
            Err(e) => {
                eprintln!("Failed to initialize tray icon: {}", e);
            }
        }
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        Greeting {}
    }
}
