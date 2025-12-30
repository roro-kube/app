/// Menu creation and management for the system tray

use tray_icon::menu::{Menu, MenuItem};

/// Creates the default system tray menu
pub fn create_tray_menu() -> Menu {
    let menu = Menu::new();
    let menu_item = MenuItem::new("About", true, None);
    menu.append(&menu_item).unwrap();
    menu
}

