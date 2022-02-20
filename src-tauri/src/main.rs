#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
    let builder = tauri::Builder::default().setup(|app| {
        let main_window = app.get_window("main").unwrap();

        main_window.open_devtools();

        Ok(())
    });

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
