#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod utils;
use commands::*;
use utils::*;

fn main() {
    let builder = tauri::Builder::default().invoke_handler(tauri::generate_handler![
        toggle_devtools,
        open_file,
        read_file,
        get_lang,
    ]);

    let config_dir = system::get_config_dir();

    builder
        .manage(CurrentFile("".into()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
