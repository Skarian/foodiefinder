// Prevents additio,nal console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod types {
    pub mod client;
    pub mod db;
    pub mod recipe;
    pub mod response;
}
mod utils {
    pub mod db;
    pub mod image;
    pub mod json;
    pub mod recipe_scrapers;
    pub mod request;
}
mod config;
mod error;

use error::{DBError, RequestError};
use tauri::AppHandle;
use tauri::Manager;
use tauri::State;
use types::client::ReqwestClient;
use types::db::Database;
use types::recipe::RecipeData;
use types::recipe::RecipeDetails;
use types::response::EdamamResponse;
use utils::image::download_image;
use utils::recipe_scrapers::scrape_recipe_from_url;
use utils::request::build_request;

fn main() {
    let client = ReqwestClient::new();

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let db = Database::new(app_handle).unwrap();
            app.manage(db);
            Ok(())
        })
        .manage(client)
        // .manage(db)
        .invoke_handler(tauri::generate_handler![
            greet,
            search_recipes,
            get_next_recipes,
            get_recipe_details,
            get_all_recipes,
            get_recipe_by_id,
            add_recipe,
            update_recipe,
            delete_recipe_by_id,
            does_recipe_exist_by_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    name.to_string()
}

#[tauri::command]
async fn search_recipes(
    query: &str,
    client: State<'_, ReqwestClient>,
) -> Result<EdamamResponse, RequestError> {
    let url = "https://api.edamam.com/api/recipes/v2";

    dotenv::dotenv().ok(); // Loads the environment variables from .env
    let app_id = std::env::var("app_id").expect("app_id must be set");
    let app_key = std::env::var("app_key").expect("app_key must be set");
    println!("app id: {app_id}");
    println!("App key: {app_key}");
    let params = maplit::hashmap! {
        "type" => "public",
        "q" => query,
        "app_id" => &app_id,
        "app_key" => &app_key
    };

    let request = build_request(url, Some(params), client.clone()).await?;

    let mut res = request.send().await?.json::<EdamamResponse>().await?;
    res.set_all_hits_scrapable_status();
    res.set_all_hits_valid_status(client).await?;

    Ok(res)
}

#[tauri::command]
async fn get_next_recipes(
    next_url: &str,
    client: State<'_, ReqwestClient>,
) -> Result<EdamamResponse, RequestError> {
    let request = build_request(next_url, None, client.clone()).await?;

    let mut res = request.send().await?.json::<EdamamResponse>().await?;
    res.set_all_hits_scrapable_status();
    res.set_all_hits_valid_status(client).await?;
    Ok(res)
}

#[tauri::command]
async fn get_recipe_details(url: &str) -> Result<RecipeData, RequestError> {
    let details = scrape_recipe_from_url(url)?;
    let json: RecipeData = serde_json::from_str(&details)?;
    Ok(json)
}

#[tauri::command]
async fn get_all_recipes(db: State<'_, Database>) -> Result<Vec<RecipeDetails>, DBError> {
    db.get_all_recipes()
}

#[tauri::command]
async fn get_recipe_by_id(
    db: State<'_, Database>,
    id: i32,
) -> Result<Option<RecipeDetails>, DBError> {
    db.get_recipe_by_id(&id)
}

#[tauri::command]
async fn add_recipe(
    client: State<'_, ReqwestClient>,
    db: State<'_, Database>,
    recipe: RecipeDetails,
    app: AppHandle,
) -> Result<(), DBError> {
    let uploaded_image = download_image(client, &recipe.image, app).await?;
    db.add_recipe(&recipe, uploaded_image)
}

#[tauri::command]
async fn update_recipe(db: State<'_, Database>, recipe: RecipeDetails) -> Result<(), DBError> {
    db.update_recipe(&recipe)
}

#[tauri::command]
async fn delete_recipe_by_id(db: State<'_, Database>, id: i32) -> Result<(), DBError> {
    db.delete_recipe_by_id(&id)
}

#[tauri::command]
async fn does_recipe_exist_by_url(db: State<'_, Database>, url: &str) -> Result<bool, DBError> {
    let status = db.does_recipe_exist_by_url(url)?;
    Ok(status)
}
