#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[macro_use]
extern crate lazy_static;
mod commands;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![
            commands::get_teachers,
            commands::get_user_info,
            commands::set_user_info,
            commands::delete,
            commands::data_dir_exists,
        ])
        .run(context)
        .expect("error while running tauri application");
}
