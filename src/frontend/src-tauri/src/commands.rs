use directories::ProjectDirs;
use keyring;
use lazy_static;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use trash;

lazy_static! {
    static ref PROJ_DIRS: ProjectDirs = ProjectDirs::from("", "", "Skootils").unwrap();
}
fn get_powerschool_dir() -> PathBuf {
    PROJ_DIRS.data_dir().join("powerschool")
}
fn get_powerschool_history_dir() -> PathBuf {
    let mut output = get_powerschool_dir();
    output.push("history");
    return output;
}
fn get_powerschool_teacher_file() -> PathBuf {
    let mut output = get_powerschool_dir();
    output.push("teachers.json");
    return output;
}
fn get_user_info_file() -> PathBuf {
    PROJ_DIRS.data_dir().join("user.json")
}

#[tauri::command]
pub fn get_teachers() -> Result<Value, String> {
    let file_path = get_powerschool_teacher_file();

    fs::read_to_string(file_path.as_path()).map_or(Err("Could not read file".into()), |read| {
        Ok(serde_json::from_str(read.as_str()).unwrap())
    })
}

#[tauri::command]
pub fn get_user_info() -> Result<Value, String> {
    let file_path = get_user_info_file();

    fs::read_to_string(file_path.as_path()).map_or(Err("Could not read file".into()), |read| {
        Ok(serde_json::from_str(read.as_str()).unwrap())
    })
}
#[tauri::command]
pub fn set_user_info(to: Value) -> Result<(), String> {
    let file_path = get_user_info_file();

    fs::write(file_path.as_path(), to.to_string()).or(Err("Could not write file".into()))
}
#[tauri::command]
pub fn delete(items: Vec<String>) -> Result<(), ()> {
    let mut to_delete = Vec::new();
    for item in items {
        match item.as_str() {
            "Your name and grade" => to_delete.push(get_user_info_file()),
            "Your credentials" => {
                // TODO: selenium one, remember?
                let key = keyring::Entry::new("Skootils", "powerschool");
                match key.delete_password() {
                    Ok(_) => (),
                    Err(_) => return Err(()),
                }
            }
            "All PowerSchool history" => to_delete.push(get_powerschool_history_dir()),
            "Teacher information" => to_delete.push(get_powerschool_teacher_file()),
            _ => panic!("bruh"),
        }
    }
    match trash::delete_all(to_delete.as_slice()) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}
