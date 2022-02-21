pub fn get_config_dir() -> &'static PathBuf {
    directories::BaseDirs::new()
        .unwrap()
        .config_dir()
        .to_path_buf()
        .push("Coders");
}
