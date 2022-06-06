
use tauri::{Menu, Submenu, MenuItem};

pub fn get_menu() -> Menu {

  let app_menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_native_item(MenuItem::Paste)
    .add_native_item(MenuItem::Separator)
    .add_native_item(MenuItem::Quit);

  Menu::new()
    .add_submenu(Submenu::new("Time Tauri", app_menu))
}
