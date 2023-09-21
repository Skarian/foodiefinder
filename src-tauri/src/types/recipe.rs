use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RecipeData {
    #[serde(rename = "canonical_url")]
    pub canonical_url: Option<String>,
    pub category: Option<String>,
    pub host: Option<String>,
    pub image: Option<String>,
    pub ingredients: Option<Vec<String>>,
    pub instructions: Option<String>,
    #[serde(rename = "instructions_list")]
    pub instructions_list: Option<Vec<String>>,
    pub language: Option<String>,
    pub ratings: Option<f64>,
    #[serde(rename = "site_name")]
    pub site_name: Option<String>,
    pub title: Option<String>,
    pub yields: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RecipeDetails {
    pub image: String,
    pub url: String,
    pub servings: i32,
    pub time: i32,
    pub calories: i32,
    pub source: Option<String>,
    pub ingredients: Option<Vec<String>>,
    pub instructions: Option<Vec<String>>,
    pub title: String,
    pub id: Option<i32>,
    pub date_added: Option<String>,
}
