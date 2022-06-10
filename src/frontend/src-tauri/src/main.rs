#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use blake3::hash;

use std::fs;
use std::path::PathBuf;

use tauri::{Menu, MenuItem, Submenu};
#[tauri::command]
fn get_from_cache(cache_dir: String, config: String) -> Result<String, String> {
    let mut cache = PathBuf::from(cache_dir);
    cache.push(String::from(hash(config.as_bytes()).to_hex().as_str()));
    fs::read_to_string(cache.as_path()).or(Err("Non existent cache".into()))
}
#[tauri::command]
fn set_cache(cache_dir: String, config: String, contents: String) {
    let mut cache = PathBuf::from(cache_dir);
    cache.push(String::from(hash(config.as_bytes()).to_hex().as_str()));
    fs::write(cache.as_path(), contents).expect("Could not write to cache")
}
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
        .invoke_handler(tauri::generate_handler![get_from_cache, set_cache])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
