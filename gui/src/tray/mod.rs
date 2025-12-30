use dioxus::desktop::trayicon::menu::Menu as TrayMenu;
/// System tray module
///
/// This module handles the system tray icon and menu functionality using
/// Dioxus Desktop's native `trayicon` module. This eliminates macOS conflicts
/// since Dioxus uses the same internal muda/tao infrastructure.
use dioxus::desktop::trayicon::{TrayIcon, TrayIconBuilder};

pub mod icon;
pub mod menu;

pub use icon::load_tray_icon;
pub use menu::create_default_tray_menu;

/// Manages the system tray icon and its lifecycle
///
/// The tray icon must be kept alive for the duration of the application.
/// Dropping this struct will remove the tray icon from the system tray.
pub struct TrayManager {
    // Field must exist to keep TrayIcon alive, even though we don't directly read it.
    // This is required for the tray icon to remain visible - dropping TrayIcon removes it from the system tray.
    #[allow(dead_code)] // Required: field keeps TrayIcon alive, dropping it removes the tray icon
    tray_icon: TrayIcon,
}

impl TrayManager {
    /// Initialize a new tray icon with default menu
    ///
    /// This creates a tray icon with the default menu configuration.
    /// The icon is loaded from embedded assets generated during build.
    pub fn init() -> Result<Self, String> {
        Self::init_with_menu(create_default_tray_menu())
    }

    /// Initialize a new tray icon with a custom menu
    ///
    /// # Arguments
    /// * `menu` - The menu to attach to the tray icon
    pub fn init_with_menu(menu: TrayMenu) -> Result<Self, String> {
        let icon = load_tray_icon().map_err(|e| format!("Failed to load tray icon: {e}"))?;

        let tray_icon = TrayIconBuilder::new()
            .with_icon(icon)
            .with_tooltip("Roro Kube")
            .with_menu(Box::new(menu))
            .build()
            .map_err(|e| format!("Failed to build tray icon: {e}"))?;

        Ok(Self { tray_icon })
    }
}

/// Initializes the system tray icon without blocking
///
/// Returns a `TrayManager` instance which must be kept alive for the
/// lifetime of the application. When dropped, the tray icon will be removed.
///
/// This function works on all platforms including macOS, since we're using
/// Dioxus's native trayicon module which handles menu class registration
/// internally.
pub fn init_tray() -> Result<TrayManager, String> {
    TrayManager::init()
}
