use std::path;

pub fn get_path_string(path: &path::PathBuf) -> String {
    path.clone().into_os_string().into_string().unwrap()
}
