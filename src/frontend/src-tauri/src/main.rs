#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[macro_use]
extern crate lazy_static;
use tauri::{Menu, MenuItem, Submenu};
mod commands;

fn main() {
    // TODO: Better menu
    let menu = Menu::new()
        .add_submenu(Submenu::new(
            "app_name",
            Menu::new()
                // .add_native_item(MenuItem::About(
                //     "app_name".to_string(),
                //     AboutMetadata::new().copyright("All rights reserved".to_string()),
                // ))
                // .add_native_item(MenuItem::Separator)
                // .add_native_item(MenuItem::Services)
                // .add_native_item(MenuItem::Separator)
                // .add_native_item(MenuItem::Hide)
                // .add_native_item(MenuItem::HideOthers)
                // .add_native_item(MenuItem::ShowAll)
                // .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Quit),
        ))
        .add_submenu(Submenu::new("Edit", {
            let mut menu = Menu::new()
                .add_native_item(MenuItem::Undo)
                .add_native_item(MenuItem::Redo)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Cut)
                .add_native_item(MenuItem::Copy)
                .add_native_item(MenuItem::Paste);
            #[cfg(not(target_os = "macos"))]
            {
                menu = menu.add_native_item(MenuItem::Separator);
            }
            menu = menu.add_native_item(MenuItem::SelectAll);
            menu
        }));
    tauri::Builder::default()
        .menu(menu)
        .invoke_handler(tauri::generate_handler![
            commands::get_teachers,
            commands::get_user_info,
            commands::set_user_info,
            commands::delete,
            commands::data_dir_exists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
