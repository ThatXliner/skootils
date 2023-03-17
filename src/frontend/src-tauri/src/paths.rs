use directories::ProjectDirs;
use lazy_static::lazy_static;
use paste::paste;
use std::fs;
use std::path::PathBuf;

lazy_static! {
    static ref PROJ_DIRS: ProjectDirs = ProjectDirs::from("", "", "Skootils").unwrap();
}

pub fn assert_dir_exists(dir: &PathBuf) {
    let path = dir.as_path();
    if !path.is_dir() {
        // This shouldn't happen unless we're being tampered with
        fs::create_dir_all(path).expect("A file was found where a directory was expected");
    }
}
macro_rules! create_dir_function {
    ($($dir:ident)/+) => {
        paste! {
            pub fn [< get_ $($dir)_+ _dir>]() -> PathBuf {
                let mut output = PROJ_DIRS.data_dir().to_owned();
                $(output.push(stringify!($dir)));+;
                assert_dir_exists(&output);
                return output;
            }
        }
    };
}
create_dir_function!(powerschool);
create_dir_function!(powerschool / history);
create_dir_function!(learnatvcs);
pub fn get_powerschool_teacher_file() -> PathBuf {
    let mut output = get_powerschool_dir();
    output.push("teachers.json");
    return output;
}
pub fn get_learnatvcs_file(filename: &str) -> PathBuf {
    let mut output = get_learnatvcs_dir();
    output.push(filename);
    return output;
}
pub fn get_user_info_file() -> PathBuf {
    assert_dir_exists(&PROJ_DIRS.data_dir().to_path_buf());
    PROJ_DIRS.data_dir().join("user.json")
}
