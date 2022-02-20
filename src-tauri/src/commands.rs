use tauri::{
    api::{dialog, file},
    Runtime,
};

#[tauri::command]
pub fn toggle_devtools<R: Runtime>(
    _: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    window.open_devtools();
    Ok(())
}

#[tauri::command]
pub fn open_file() -> Result<String, String> {
    let file_path = dialog::blocking::FileDialogBuilder::new().pick_file();

    match file_path {
        Some(path) => Ok(path.into_os_string().into_string().unwrap()),
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
