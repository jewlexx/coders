use std::path::Path;

pub fn get_config_dir() -> &'static Path {
    directories::ProjectDirs::from("com", "jewlexx", "Coders")
        .unwrap()
        .project_path()
}
