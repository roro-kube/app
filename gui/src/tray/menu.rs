/// Menu creation and management for the system tray
/// 
/// Uses Dioxus Desktop's native menu types which integrate seamlessly
/// with the Dioxus event system.

use dioxus::desktop::trayicon::menu::{Menu, MenuItem};

/// Builder for creating system tray menus
pub struct TrayMenuBuilder {
    items: Vec<MenuItem>,
}

impl TrayMenuBuilder {
    /// Create a new menu builder
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add a menu item to the menu
    /// 
    /// # Arguments
    /// * `label` - The text label for the menu item
    /// * `enabled` - Whether the menu item is enabled
    /// * `_id` - Optional unique identifier for the menu item (currently unused, reserved for future event handling)
    pub fn add_item(mut self, label: &str, enabled: bool, _id: Option<&str>) -> Self {
        // MenuItem::new takes (label, enabled, accelerator) where accelerator is Option<Accelerator>
        let menu_item = MenuItem::new(label, enabled, None);
        self.items.push(menu_item);
        self
    }

    /// Add a separator to the menu
    /// 
    /// Note: Separator support may be limited depending on the Dioxus menu API.
    /// This is a placeholder for future functionality.
    #[allow(dead_code)]
    pub fn add_separator(self) -> Self {
        // Separator functionality not yet implemented
        // Menu items can be added with empty labels as a workaround if needed
        self
    }

    /// Build the menu from the configured items
    /// 
    /// Returns the constructed Menu that can be attached to a tray icon.
    pub fn build(self) -> Menu {
        let menu = Menu::new();
        for item in self.items {
            if let Err(e) = menu.append(&item) {
                eprintln!("Failed to append menu item: {}", e);
            }
        }
        menu
    }
}

impl Default for TrayMenuBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates the default system tray menu
/// 
/// This is a convenience function that creates a basic menu with common items.
/// For more control, use `TrayMenuBuilder` directly.
pub fn create_default_tray_menu() -> Menu {
    TrayMenuBuilder::new()
        .add_item("About", true, Some("about"))
        .build()
}
