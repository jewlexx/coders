use std::path::PathBuf;

pub fn get_config_dir() -> PathBuf {
    directories::ProjectDirs::from("com", "jewlexx", "Coders")
        .unwrap()
        .project_path()
        .to_path_buf()
}
