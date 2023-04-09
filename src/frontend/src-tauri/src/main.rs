#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod learnatvcs;
mod powerschool;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![
            powerschool::get_teachers,
            powerschool::get_user_info,
            powerschool::set_user_info,
            powerschool::delete,
            powerschool::data_dir_exists,
            powerschool::add_teacher,
            powerschool::remove_teacher,
            learnatvcs::scrape_plans,
        ])
        .run(context)
        .expect("error while running tauri application");
}
