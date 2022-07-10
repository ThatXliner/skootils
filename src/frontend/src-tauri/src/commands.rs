use directories::ProjectDirs;
use lazy_static;
use serde_json::Value;
use std::fs;
// use trash;

// DISCUSS: Would using lazy static
// And statically link break things
// When we're at the packaging step?
lazy_static! {
    static ref PROJ_DIRS: ProjectDirs = ProjectDirs::from("", "", "Skootils").unwrap();
}
#[tauri::command]
pub fn get_teachers() -> Result<Value, String> {
    let mut file_path = PROJ_DIRS.data_dir().join("powerschool");
    file_path.push("teachers.json");

    fs::read_to_string(file_path.as_path()).map_or(Err("Could not read file".into()), |read| {
        Ok(serde_json::from_str(read.as_str()).unwrap())
    })
}
#[tauri::command]
pub fn get_user_info() -> Result<Value, String> {
    let file_path = PROJ_DIRS.data_dir().join("user.json");

    fs::read_to_string(file_path.as_path()).map_or(Err("Could not read file".into()), |read| {
        Ok(serde_json::from_str(read.as_str()).unwrap())
    })
}
#[tauri::command]
pub fn set_user_info(to: Value) -> Result<(), String> {
    let file_path = PROJ_DIRS.data_dir().join("user.json");

    fs::write(file_path.as_path(), to.to_string()).or(Err("Could not write file".into()))
}
// #[tauri::command]
// pub fn delete(items:Vec<String>){
//     for item in items {
//         match item.as_str() {
//             "Your name and grade" => trash::delete(PROJ_DIRS.data_dir().join("user.json").as_path()),
//             "Your credentials" => todo!(),// TODO
//             "All PowerSchool history" => trash::delete
//         }
//     }
// }
