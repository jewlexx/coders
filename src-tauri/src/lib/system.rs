use std::path::PathBuf;

lazy_static! {
    pub static ref PROJECT_DIRS: directories::ProjectDirs =
        directories::ProjectDirs::from("com", "jewlexx", "Coders").unwrap();
}

pub fn get_config_dir() -> PathBuf {
    PROJECT_DIRS.config_dir().to_path_buf()
}
