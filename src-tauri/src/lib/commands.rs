// use serde_yaml::{from_str, Mapping, Value};
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

pub struct CurrentFile(pub String);

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
            Ok(path.into_os_string().into_string().unwrap())
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
    // let client = tauri::api::http::ClientBuilder::new().build().unwrap();
    // let request = tauri::api::http::HttpRequestBuilder::new(
    //     "get",
    //     "https://raw.githubusercontent.com/github/linguist/master/lib/linguist/languages.yml",
    // )
    // .unwrap();

    // let result = client.send(request).await.unwrap();
    // let bytes = result.bytes().await.unwrap().data;
    // let string = std::str::from_utf8(&bytes).unwrap().to_string();
    // let value: Mapping = from_str(&string).unwrap();

    // for (k, v) in value {
    //     match &v["extensions"] {
    //         Value::Null => continue,
    //         Value::Bool(_) => continue,
    //         Value::Number(_) => continue,
    //         Value::String(_) => continue,
    //         Value::Mapping(_) => continue,
    //         Value::Sequence(s) => {
    //             for ext in s {
    //                 if ext.as_str().unwrap() == format!(".{}", file_path.split('.').last().unwrap())
    //                 {
    //                     println!("{}", k.as_str().unwrap());
    //                     return Ok(k.as_str().unwrap().to_string().to_lowercase());
    //                 }
    //             }
    //         }
    //     }
    // }

    match detect_lang::from_extension(file_path.split('.').last().unwrap()) {
        Some(lang) => Ok(lang.to_string().to_lowercase()),
        None => Ok("unknown".to_string()),
    }
}
