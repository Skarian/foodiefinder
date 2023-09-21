use crate::error::ConfigPathError;
use std::{fs::create_dir, path::PathBuf};
use tauri::AppHandle;

pub fn get_or_create_app_dir(app: AppHandle) -> Result<PathBuf, ConfigPathError> {
    // Tauri suggested data dir
    let data_dir = app.path_resolver().app_data_dir();
    // Kicks out error if tauri unable to find home dir
    match data_dir {
        // Checks if the tauri suggested data dir exists or not, if it does return that
        // path, if it doesn't create it and then return it
        Some(dir) => match dir.is_dir() {
            true => Ok(dir),
            false => {
                create_dir(&dir)?;
                Ok(dir)
            }
        },
        None => Err(ConfigPathError::HomeDirNotFound),
    }
}
pub fn get_or_create_db_path(app: AppHandle) -> Result<PathBuf, ConfigPathError> {
    let app_dir = get_or_create_app_dir(app)?;
    Ok(app_dir.join("recipes.db"))
}

pub fn get_or_create_images_path(app: AppHandle) -> Result<PathBuf, ConfigPathError> {
    let data_dir = get_or_create_app_dir(app)?;
    let image_path = data_dir.join("images/");

    match image_path.is_dir() {
        true => Ok(image_path),
        false => {
            create_dir(&image_path)?;
            Ok(image_path)
        }
    }
}
