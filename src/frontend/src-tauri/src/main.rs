#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use directories::ProjectDirs;
#[macro_use]
extern crate lazy_static;
use serde_json::Value;
use std::fs;
use tauri::{Menu, MenuItem, Submenu};

lazy_static! {
    static ref PROJ_DIRS: ProjectDirs = ProjectDirs::from("", "", "Skootils").unwrap();
}
#[tauri::command]
fn get_teachers() -> Result<Value, String> {
    let mut file_path = PROJ_DIRS.data_dir().join("powerschool");
    file_path.push("teachers.json");

    fs::read_to_string(file_path.as_path()).map_or(Err("Could not read file".into()), |read| {
        Ok(serde_json::from_str(read.as_str()).unwrap())
    })
}
#[tauri::command]
fn get_user_info() -> Result<Value, String> {
    let file_path = PROJ_DIRS.data_dir().join("user.json");

    fs::read_to_string(file_path.as_path()).map_or(Err("Could not read file".into()), |read| {
        Ok(serde_json::from_str(read.as_str()).unwrap())
    })
}
#[tauri::command]
fn set_user_info(to: Value) -> Result<(), String> {
    let file_path = PROJ_DIRS.data_dir().join("user.json");

    fs::write(file_path.as_path(), to.to_string()).or(Err("Could not write file".into()))
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
        .invoke_handler(tauri::generate_handler![
            get_teachers,
            get_user_info,
            set_user_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
