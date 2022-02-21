use std::fs::write;

use tauri::{
    api::{dialog, file},
    Runtime,
};

use super::super::lang;
use super::system::get_config_dir;

#[tauri::command]
pub fn toggle_devtools<R: Runtime>(
    _: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    window.open_devtools();
    Ok(())
}

pub struct CurrentFile(pub String);

#[tauri::command]
pub fn get_old_file(state: tauri::State<'_, CurrentFile>) -> Result<String, String> {
    Ok(state.0.clone())
}

#[tauri::command]
pub fn open_file<R: Runtime>(
    _: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<String, String> {
    let file_path = dialog::blocking::FileDialogBuilder::new().pick_file();

    match file_path {
        Some(path) => {
            let path = path;
            window
                .set_title(&format!(
                    "Coders | {}",
                    path.file_name().unwrap().to_str().unwrap()
                ))
                .unwrap();
            let file_path = path.into_os_string().into_string().unwrap();

            match write(get_config_dir().join("workspace.json"), &file_path) {
                Ok(f) => f,
                Err(_) => println!("Failed to write to config file"),
            };

            Ok(file_path)
        }
        None => Ok("Did not choose file".into()),
    }
}

#[tauri::command]
pub async fn read_file(file_path: String) -> Result<String, String> {
    match file::read_string(file_path) {
        Ok(content) => Ok(content),
        Err(err) => Err(format!("{}", err)),
    }
}

#[tauri::command]
pub async fn get_lang(file_path: String) -> Result<String, String> {
    match lang::from_extension(file_path.split('.').last().unwrap()) {
        Some(lang) => Ok(lang.name().to_lowercase()),
        None => Ok("unknown".to_string()),
    }
}
