use std::path::PathBuf;

pub fn get_config_dir() -> PathBuf {
    let mut path = directories::BaseDirs::new()
        .unwrap()
        .config_dir()
        .to_path_buf();

    path.push("Coders");

    path.clone()
}
