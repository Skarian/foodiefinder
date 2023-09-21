use crate::config::get_or_create_images_path;
use crate::error::{DBError, RequestError};
use crate::types::client::ReqwestClient;
use crate::utils::request::build_request;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tauri::AppHandle;
use url::Url;

pub async fn download_image(
    client: tauri::State<'_, ReqwestClient>,
    image: &str,
    app: AppHandle,
) -> Result<Option<String>, DBError> {
    let image_extension = match get_extension_from_url(image) {
        Some(extension) => extension,
        None => return Ok(None),
    };

    let image_name = format!("{}.{}", uuid::Uuid::new_v4(), image_extension);

    let image_path = format!(
        "{}{}",
        get_or_create_images_path(app)?.display(),
        image_name
    );

    let request = build_request(image, None, client.clone()).await?;
    let response = request.send().await.map_err(RequestError::Middleware)?;

    match response.status().is_success() {
        true => {
            let bytes = response.bytes().await.map_err(RequestError::Reqwest)?;
            let mut file = File::create(Path::new(&image_path))?;
            file.write_all(&bytes)?;
            Ok(Some(image_path))
        }
        false => Ok(None),
    }
}

fn get_extension_from_url(url: &str) -> Option<String> {
    let parsed_url = Url::parse(url).ok()?;
    let path = parsed_url.path();
    Path::new(path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from)
}
