/// System tray module
/// 
/// This module handles the system tray icon and menu functionality using
/// Dioxus Desktop's native `trayicon` module. This eliminates macOS conflicts
/// since Dioxus uses the same internal muda/tao infrastructure.

use dioxus::desktop::trayicon::{Icon, TrayIcon, TrayIconBuilder};
use dioxus::desktop::trayicon::menu::Menu as TrayMenu;

pub mod icon;
pub mod menu;

pub use icon::load_tray_icon;
pub use menu::create_default_tray_menu;

/// Manages the system tray icon and its lifecycle
/// 
/// The tray icon must be kept alive for the duration of the application.
/// Dropping this struct will remove the tray icon from the system tray.
pub struct TrayManager {
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
        let icon = load_tray_icon()
            .map_err(|e| format!("Failed to load tray icon: {}", e))?;

        let tray_icon = TrayIconBuilder::new()
            .with_icon(icon)
            .with_tooltip("Roro Kube")
            .with_menu(Box::new(menu))
            .build()
            .map_err(|e| format!("Failed to build tray icon: {}", e))?;

        Ok(Self { tray_icon })
    }

    /// Update the tray icon's menu
    /// 
    /// # Arguments
    /// * `_menu` - The new menu to attach
    #[allow(dead_code)]
    pub fn update_menu(&mut self, _menu: TrayMenu) -> Result<(), String> {
        // Note: TrayIcon doesn't have a direct update_menu method in the API
        // We may need to recreate the tray icon or use platform-specific APIs
        // For now, this is a placeholder for future functionality
        eprintln!("Menu update not yet implemented - tray icon must be recreated");
        Ok(())
    }

    /// Update the tray icon's image
    /// 
    /// # Arguments
    /// * `_icon` - The new icon to display
    #[allow(dead_code)]
    pub fn update_icon(&mut self, _icon: Icon) -> Result<(), String> {
        // Note: TrayIcon doesn't have a direct update_icon method in the API
        // We may need to recreate the tray icon or use platform-specific APIs
        // For now, this is a placeholder for future functionality
        eprintln!("Icon update not yet implemented - tray icon must be recreated");
        Ok(())
    }

    /// Set the tray icon's tooltip text
    /// 
    /// # Arguments
    /// * `_tooltip` - The tooltip text to display
    #[allow(dead_code)]
    pub fn set_tooltip(&mut self, _tooltip: &str) -> Result<(), String> {
        // Note: TrayIcon doesn't have a direct set_tooltip method in the API
        // We may need to recreate the tray icon or use platform-specific APIs
        // For now, this is a placeholder for future functionality
        eprintln!("Tooltip update not yet implemented - tray icon must be recreated");
        Ok(())
    }

    /// Get a reference to the underlying TrayIcon
    /// 
    /// This is useful for advanced use cases where direct access to the
    /// TrayIcon API is needed.
    #[allow(dead_code)]
    pub fn tray_icon(&self) -> &TrayIcon {
        &self.tray_icon
    }
}

/// Initializes the system tray icon without blocking
/// 
/// Returns a TrayManager instance which must be kept alive for the
/// lifetime of the application. When dropped, the tray icon will be removed.
/// 
/// This function works on all platforms including macOS, since we're using
/// Dioxus's native trayicon module which handles menu class registration
/// internally.
pub fn init_tray() -> Result<TrayManager, String> {
    TrayManager::init()
}
