use scraper::{Html, Selector};
use serde_json::Value;

use crate::error::RecipeJsonError;

/// Determines whether the provided HTML body contains a recipe.
///
/// The function parses the HTML document, searches for script tags with the type "application/ld+json",
/// and extracts their inner HTML content. It then performs the following steps:
///
/// 1. `remove_escape_control_characters`: Removes all control characters from the extracted HTML content.
/// 2. `serde_json::from_str`: Attempts to parse the processed content as JSON.
/// 3. `is_recipe`: Checks if the parsed JSON represents a recipe by recursively traversing the JSON structure
///    and looking for objects with a property "@type" set to the value "recipe" (case-insensitive).
///
/// If a recipe is found at any step, the function returns `Ok(true)`.
/// If no recipe is found, the function returns `Ok(false)`.
/// If there is an error parsing the HTML or JSON, the function returns `Err(RecipeJsonError)`.
///
/// # Arguments
///
/// * `body` - A string representing the HTML body to search for recipe JSON.
///
/// # Returns
///
/// A result indicating whether the HTML body contains a recipe:
///
/// * `Ok(true)` - If the HTML body contains at least one JSON value that represents a recipe.
/// * `Ok(false)` - If the HTML body does not contain any JSON values representing a recipe or if there is an error parsing the JSON.
/// * `Err(RecipeJsonError)` - If there is an error parsing the HTML or extracting the JSON content.
///
/// # Errors
///
/// The function can return a `RecipeJsonError` if there is an error parsing the HTML or extracting the JSON content.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use serde_json::Value;
///
/// let html_body = fs::read_to_string("recipes.html").expect("Failed to read HTML file.");
/// let has_recipe = has_recipe(html_body);
///
/// match has_recipe {
///     Ok(true) => println!("The HTML body contains a recipe!"),
///     Ok(false) => println!("No recipes found in the HTML body."),
///     Err(err) => eprintln!("Error: {}", err),
/// }
/// ```
pub fn has_recipe(body: String) -> Result<bool, RecipeJsonError> {
    let html = Html::parse_document(&body);
    let selector = Selector::parse(r#"script[type="application/ld+json"]"#)
        .map_err(|_| RecipeJsonError::HtmlParsing)?;

    fn is_recipe(json: &Value) -> bool {
        match json {
            Value::Object(obj_json) => obj_json.get("@type").map_or(false, check_recipe),
            Value::Array(arr_json) => arr_json.iter().any(is_recipe),
            _ => false,
        }
    }

    fn check_recipe(entry: &Value) -> bool {
        match entry {
            Value::String(entry_string) => entry_string.trim().to_lowercase() == "recipe",
            Value::Array(entry_array) => entry_array.iter().any(|entry| match entry {
                Value::String(entry_string) => entry_string.trim().to_lowercase() == "recipe",
                _ => false,
            }),
            _ => false,
        }
    }

    fn remove_escape_control_characters(input: &str) -> String {
        input
            .chars()
            .filter(|c| !c.is_control())
            .collect::<String>()
    }

    for entry in html.select(&selector) {
        let content = entry.inner_html();
        let processed_content = remove_escape_control_characters(&content);
        let json: Value = serde_json::from_str(&processed_content)?;
        if is_recipe(&json) {
            return Ok(true);
        }
    }

    Ok(false)
}
