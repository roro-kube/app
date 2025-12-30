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
mod pages;
mod layout;

fn main() {
    // Initialize and run the system tray application
    // All business logic is delegated to the Core layer
    tray::run_tray_app();
}

