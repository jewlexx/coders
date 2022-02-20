#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Runtime;

#[tauri::command]
async fn toggle_devtools<R: Runtime>(
    _: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    window.open_devtools();
    Ok(())
}

fn main() {
    let builder =
        tauri::Builder::default().invoke_handler(tauri::generate_handler![toggle_devtools]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
