use directories::ProjectDirs;
use keyring;
use lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
#[derive(Serialize, Deserialize)]
pub struct TeacherInfo {
    email: String,
    class_name: String,
    period: String,
}

use std::fs;
use std::path::{Path, PathBuf};
use trash;

lazy_static! {
    static ref PROJ_DIRS: ProjectDirs = ProjectDirs::from("", "", "Skootils").unwrap();
}
fn assert_dir_exists(dir: &PathBuf) {
    let path = dir.as_path();
    if !path.is_dir() {
        // This shouldn't happen unless we're being tampered with
        fs::create_dir_all(path).expect("A file was found where a directory was expected");
    }
}
fn get_powerschool_dir() -> PathBuf {
    let output = PROJ_DIRS.data_dir().join("powerschool");
    assert_dir_exists(&output);
    return output;
}
fn get_learnatvcs_dir() -> PathBuf {
    let output = PROJ_DIRS.data_dir().join("learnatvcs");
    assert_dir_exists(&output);
    return output;
}
fn get_powerschool_history_dir() -> PathBuf {
    let mut output = get_powerschool_dir();
    output.push("history");
    assert_dir_exists(&output);
    return output;
}
fn get_powerschool_teacher_file() -> PathBuf {
    let mut output = get_powerschool_dir();
    output.push("teachers.json");
    return output;
}
fn get_learnatvcs_file(filename: &str) -> PathBuf {
    let mut output = get_learnatvcs_dir();
    output.push(filename);
    return output;
}
fn get_user_info_file() -> PathBuf {
    assert_dir_exists(&PROJ_DIRS.data_dir().to_path_buf());
    PROJ_DIRS.data_dir().join("user.json")
}
fn read_json(file: &Path) -> Result<Value, String> {
    fs::read_to_string(file).map_or(Err("Could not read file".into()), |read| {
        serde_json::from_str(read.as_str()).or(Err("Could not parse JSON".into()))
    })
}
#[tauri::command]
pub fn get_teachers() -> Result<Value, String> {
    let file_path = get_powerschool_teacher_file();
    read_json(file_path.as_path())
}
#[tauri::command]
pub fn add_teacher(teacher_name: String, teacher_info: TeacherInfo) -> Result<(), String> {
    let file_path = get_powerschool_teacher_file();
    let mut val: HashMap<String, TeacherInfo> = serde_json::from_str(
        fs::read_to_string(&file_path)
            .unwrap_or_else(|_| String::from("{}"))
            .as_str(),
    )
    .or(Err(String::from("Could not parse JSON")))?;
    val.insert(teacher_name, teacher_info);
    fs::write(
        &file_path,
        serde_json::to_string(&val).or(Err(String::from("Converting to string failed")))?,
    )
    .or(Err(String::from("Could not write to file")))?;
    Ok(())
}
#[tauri::command]
pub fn remove_teacher(teacher_name: String) -> Result<(), String> {
    let file_path = get_powerschool_teacher_file();
    let mut val: HashMap<String, TeacherInfo> = serde_json::from_str(
        fs::read_to_string(&file_path)
            .unwrap_or_else(|_| String::from("{}"))
            .as_str(),
    )
    .or(Err(String::from("Could not parse JSON")))?;
    val.remove(&teacher_name);
    fs::write(
        &file_path,
        serde_json::to_string(&val).or(Err(String::from("Converting to string failed")))?,
    )
    .or(Err(String::from("Could not write to file")))?;
    Ok(())
}
#[tauri::command]
pub fn get_user_info() -> Result<Value, String> {
    let file_path = get_user_info_file();
    read_json(file_path.as_path())
}
#[tauri::command]
pub fn set_user_info(to: Value) -> Result<(), String> {
    let file_path = get_user_info_file();

    fs::write(file_path.as_path(), to.to_string()).or(Err("Could not write file".into()))
}
#[tauri::command]
pub fn delete(items: Vec<String>) -> Result<(), String> {
    let mut to_delete = Vec::new();
    for item in items {
        match item.as_str() {
            "Your name and grade" => to_delete.push(get_user_info_file()),
            "Your credentials" => {
                // TODO: selenium one, remember?
                let key = keyring::Entry::new("skootils", "powerschool");
                match key.delete_password() {
                    Ok(_) => (),
                    Err(_) => return Err("Could not delete credentials".into()),
                }
            }
            "All PowerSchool history" => to_delete.push(get_powerschool_history_dir()),
            "Teacher information" => to_delete.push(get_powerschool_teacher_file()),
            "learn@vcs scraping profile" => to_delete.push(get_learnatvcs_file("profile")),
            "learn@vcs bundled chromedriver" => to_delete.push(get_learnatvcs_file("chromedriver")),
            _ => return Err("Internal error. Please report this to the developers".into()),
        }
    }
    match trash::delete_all(to_delete.as_slice()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Could not delete stuff".into()),
    }
}
#[tauri::command]
pub fn data_dir_exists(name: String) -> bool {
    return match name.as_str() {
        "Your name and grade" => get_user_info_file().as_path().exists(),
        "Your credentials" => {
            // TODO: selenium one, remember?
            let key = keyring::Entry::new("skootils", "powerschool");
            key.get_password().is_ok()
        }
        "All PowerSchool history" => get_powerschool_history_dir().as_path().exists(),
        "Teacher information" => get_powerschool_teacher_file().as_path().exists(),
        "learn@vcs scraping profile" => get_learnatvcs_file("profile").as_path().exists(),
        "learn@vcs bundled chromedriver" => get_learnatvcs_file("chromedriver").as_path().exists(),
        _ => false, // Default to non-existent
                    // Because that way, bugs will be easier to catch
    };
}
