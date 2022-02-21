#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

mod lib;
use lib::{commands::*, system::*};

use std::fs;

fn main() {
    let builder = tauri::Builder::default().invoke_handler(tauri::generate_handler![
        toggle_devtools,
        open_file,
        read_file,
        get_lang,
    ]);

    let config_dir = get_config_dir();
    if config_dir.exists() {
        if !config_dir.is_dir() {
            fs::remove_file(config_dir)
                .expect("Failed to remove config file, please remove it manually");
        }
    } else {
        fs::create_dir_all(config_dir).expect("Failed to create config directory");
    }

    builder
        .manage(CurrentFile("".into()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
