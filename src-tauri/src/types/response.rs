use futures::{stream, StreamExt};
use psl::domain_str;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use ts_rs::TS;
use url::Url;

use crate::{
    error::{CheckScrapableError, GetHitBodiesError, SetAllHitsValidError},
    utils::json::has_recipe,
};

use super::client::ReqwestClient;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct EdamamResponse {
    pub from: i64,
    pub to: i64,
    pub count: i64,
    #[serde(rename = "_links")]
    pub links: Option<Links>,
    pub hits: Vec<Hit>,
}

impl EdamamResponse {
    fn check_if_scrapable(
        url: &str,
        scrapable_domains: &[String],
    ) -> Result<bool, CheckScrapableError> {
        let parsed_url = Url::parse(url).map_err(CheckScrapableError::UrlParseError)?;
        let host = parsed_url.host_str().ok_or(CheckScrapableError::NoHost)?;
        let domain_name = domain_str(host).ok_or(CheckScrapableError::NoDomain)?;
        Ok(scrapable_domains.contains(&domain_name.to_owned()))
    }

    /// Updates the scrapable status of all search `hits` in the instance.
    ///
    /// This method reads from a text file of valid hosts, and checks whether each hit's recipe URL
    /// is scrapable based on whether its domain is present in the valid hosts. It then sets the
    /// scrapable status of the hit accordingly. If an error occurs while checking the scrapability
    /// of a URL, it logs the error and sets the hit's scrapable status to `false`.
    ///
    /// The path to the text file of valid hosts is `../../scripts/get_valid_hosts/output_hosts.txt`.
    /// The file should contain one host per line.
    ///
    /// # Errors
    ///
    /// Logs an error message if there is a problem checking if a URL is scrapable.
    ///
    /// # Side Effects
    ///
    /// Mutates the `hits` field of the instance, specifically updating the `scrapable` status of
    /// each `hit`.
    ///
    pub fn set_all_hits_scrapable_status(&mut self) {
        let scrapable_domains: Vec<String> =
            include_str!("../../scripts/get_valid_hosts/output_hosts.txt")
                .lines()
                .map(String::from)
                .collect();

        for hit in &mut self.hits {
            match EdamamResponse::check_if_scrapable(&hit.recipe.url, &scrapable_domains) {
                Ok(scrapable) => {
                    hit.set_scrapable(scrapable);
                }
                Err(error) => {
                    println!(
                        "Error checking if URL is scrapable {}: {}",
                        &hit.recipe.url, error
                    );

                    hit.set_scrapable(false);
                }
            }
        }
    }

    pub async fn set_all_hits_valid_status(
        &mut self,
        client: tauri::State<'_, ReqwestClient>,
    ) -> Result<(), SetAllHitsValidError> {
        const CONCURRENT_REQUESTS: usize = 20;
        let client = client.0.clone();
        let validity = self
            .get_hit_bodies(client)
            .await
            .map_err(|_| SetAllHitsValidError::GetHitBodiesFailed)?;
        let hits = Arc::new(Mutex::new(&mut self.hits));
        let bodies: Vec<_> = validity.into_iter().enumerate().collect();

        let statuses: Vec<Result<bool, _>> =
            futures::stream::iter(bodies.into_iter().map(|(index, body)| {
                let hits = Arc::clone(&hits);
                async move {
                    match has_recipe(body.to_string()) {
                        Ok(status) => {
                            let mut hits = hits
                                .lock()
                                .map_err(|_| SetAllHitsValidError::MutexPoisoned)?;
                            hits[index].set_valid(status);
                            Ok(status)
                        }
                        Err(_) => {
                            let mut hits = hits
                                .lock()
                                .map_err(|_| SetAllHitsValidError::MutexPoisoned)?;
                            hits[index].set_valid(false);
                            Err(SetAllHitsValidError::SetStatusFailed)
                        }
                    }
                }
            }))
            .buffered(CONCURRENT_REQUESTS)
            .collect()
            .await;

        for status in statuses {
            status?;
        }

        Ok(())
    }

    async fn get_hit_bodies(
        &mut self,
        client: ClientWithMiddleware,
    ) -> Result<Vec<String>, GetHitBodiesError> {
        const CONCURRENT_REQUESTS: usize = 20;

        // Generates list of URLS from hits
        let urls: Vec<_> = self
            .hits
            .iter()
            .map(|hit| hit.recipe.url.as_str())
            .collect();

        // Generates list of requests using a single client
        let requests: Result<Vec<_>, _> = urls
            .into_iter()
            .map(|url| client.get(url).build())
            .collect();

        let requests = requests?;

        let bodies = stream::iter(requests.into_iter())
            .map(|request| {
                let client = client.clone();
                async move {
                    let resp = client.execute(request).await;
                    match resp {
                        Ok(response) => match response.text().await {
                            Ok(text) => Ok(text),
                            Err(_) => Ok("".to_owned()),
                        },
                        Err(_) => Ok("".to_owned()),
                    }
                }
            })
            .buffered(CONCURRENT_REQUESTS);

        let results: Vec<_> = bodies.collect().await;
        let bodies: Result<Vec<String>, _> = results.into_iter().collect();
        bodies
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Links {
    pub next: Option<Next>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Next {
    pub href: String,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Hit {
    pub recipe: Recipe,
    #[serde(rename = "_links")]
    pub links: Links2,
    pub is_scrapable: Option<bool>,
    pub is_valid: Option<bool>,
}

impl Hit {
    // A method to change the value of `is_scrapable`.
    pub fn set_scrapable(&mut self, is_scrapable: bool) {
        self.is_scrapable = Some(is_scrapable);
    }

    pub fn set_valid(&mut self, is_valid: bool) {
        self.is_valid = Some(is_valid);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Recipe {
    pub uri: String,
    pub label: String,
    pub image: String,
    pub images: Images,
    pub source: String,
    pub url: String,
    pub share_as: String,
    #[serde(rename = "yield")]
    pub yield_field: f64,
    pub diet_labels: Vec<String>,
    pub health_labels: Vec<String>,
    pub cautions: Vec<String>,
    pub ingredient_lines: Vec<String>,
    pub ingredients: Vec<Ingredient>,
    pub calories: f64,
    pub total_weight: f64,
    pub total_time: f64,
    pub cuisine_type: Vec<String>,
    pub meal_type: Vec<String>,
    #[serde(default)]
    pub dish_type: Vec<String>,
    pub total_nutrients: TotalNutrients,
    pub total_daily: TotalDaily,
    pub digest: Vec<Digest>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Images {
    #[serde(rename = "THUMBNAIL")]
    pub thumbnail: Option<Image>,
    #[serde(rename = "SMALL")]
    pub small: Option<Image>,
    #[serde(rename = "REGULAR")]
    pub regular: Option<Image>,
    #[serde(rename = "LARGE")]
    pub large: Option<Image>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Image {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Ingredient {
    pub text: Option<String>,
    pub quantity: Option<f64>,
    pub measure: Option<String>,
    pub food: Option<String>,
    pub weight: Option<f64>,
    pub food_category: Option<String>,
    pub food_id: Option<String>,
    pub image: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TotalNutrients {
    #[serde(rename = "ENERC_KCAL")]
    pub enerc_kcal: Option<Nutrient>,
    #[serde(rename = "FAT")]
    pub fat: Option<Nutrient>,
    #[serde(rename = "FASAT")]
    pub fasat: Option<Nutrient>,
    #[serde(rename = "FATRN")]
    pub fatrn: Option<Nutrient>,
    #[serde(rename = "FAMS")]
    pub fams: Option<Nutrient>,
    #[serde(rename = "FAPU")]
    pub fapu: Option<Nutrient>,
    #[serde(rename = "CHOCDF")]
    pub chocdf: Option<Nutrient>,
    #[serde(rename = "CHOCDF.net")]
    pub chocdf_net: Option<Nutrient>,
    #[serde(rename = "FIBTG")]
    pub fibtg: Option<Nutrient>,
    #[serde(rename = "SUGAR")]
    pub sugar: Option<Nutrient>,
    #[serde(rename = "PROCNT")]
    pub procnt: Option<Nutrient>,
    #[serde(rename = "CHOLE")]
    pub chole: Option<Nutrient>,
    #[serde(rename = "NA")]
    pub na: Option<Nutrient>,
    #[serde(rename = "CA")]
    pub ca: Option<Nutrient>,
    #[serde(rename = "MG")]
    pub mg: Option<Nutrient>,
    #[serde(rename = "K")]
    pub k: Option<Nutrient>,
    #[serde(rename = "FE")]
    pub fe: Option<Nutrient>,
    #[serde(rename = "ZN")]
    pub zn: Option<Nutrient>,
    #[serde(rename = "P")]
    pub p: Option<Nutrient>,
    #[serde(rename = "VITA_RAE")]
    pub vita_rae: Option<Nutrient>,
    #[serde(rename = "VITC")]
    pub vitc: Option<Nutrient>,
    #[serde(rename = "THIA")]
    pub thia: Option<Nutrient>,
    #[serde(rename = "RIBF")]
    pub ribf: Option<Nutrient>,
    #[serde(rename = "NIA")]
    pub nia: Option<Nutrient>,
    #[serde(rename = "VITB6A")]
    pub vitb6a: Option<Nutrient>,
    #[serde(rename = "FOLDFE")]
    pub foldfe: Option<Nutrient>,
    #[serde(rename = "FOLFD")]
    pub folfd: Option<Nutrient>,
    #[serde(rename = "FOLAC")]
    pub folac: Option<Nutrient>,
    #[serde(rename = "VITB12")]
    pub vitb12: Option<Nutrient>,
    #[serde(rename = "VITD")]
    pub vitd: Option<Nutrient>,
    #[serde(rename = "TOCPHA")]
    pub tocpha: Option<Nutrient>,
    #[serde(rename = "VITK1")]
    pub vitk1: Option<Nutrient>,
    #[serde(rename = "WATER")]
    pub water: Option<Nutrient>,
    #[serde(rename = "SUGAR.added")]
    pub sugar_added: Option<Nutrient>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Nutrient {
    pub label: String,
    pub quantity: f64,
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TotalDaily {
    #[serde(rename = "ENERC_KCAL")]
    pub enerc_kcal: Option<Nutrient>,
    #[serde(rename = "FAT")]
    pub fat: Option<Nutrient>,
    #[serde(rename = "FASAT")]
    pub fasat: Option<Nutrient>,
    #[serde(rename = "CHOCDF")]
    pub chocdf: Option<Nutrient>,
    #[serde(rename = "FIBTG")]
    pub fibtg: Option<Nutrient>,
    #[serde(rename = "PROCNT")]
    pub procnt: Option<Nutrient>,
    #[serde(rename = "CHOLE")]
    pub chole: Option<Nutrient>,
    #[serde(rename = "NA")]
    pub na: Option<Nutrient>,
    #[serde(rename = "CA")]
    pub ca: Option<Nutrient>,
    #[serde(rename = "MG")]
    pub mg: Option<Nutrient>,
    #[serde(rename = "K")]
    pub k: Option<Nutrient>,
    #[serde(rename = "FE")]
    pub fe: Option<Nutrient>,
    #[serde(rename = "ZN")]
    pub zn: Option<Nutrient>,
    #[serde(rename = "P")]
    pub p: Option<Nutrient>,
    #[serde(rename = "VITA_RAE")]
    pub vita_rae: Option<Nutrient>,
    #[serde(rename = "VITC")]
    pub vitc: Option<Nutrient>,
    #[serde(rename = "THIA")]
    pub thia: Option<Nutrient>,
    #[serde(rename = "RIBF")]
    pub ribf: Option<Nutrient>,
    #[serde(rename = "NIA")]
    pub nia: Option<Nutrient>,
    #[serde(rename = "VITB6A")]
    pub vitb6a: Option<Nutrient>,
    #[serde(rename = "FOLDFE")]
    pub foldfe: Option<Nutrient>,
    #[serde(rename = "VITB12")]
    pub vitb12: Option<Nutrient>,
    #[serde(rename = "VITD")]
    pub vitd: Option<Nutrient>,
    #[serde(rename = "TOCPHA")]
    pub tocpha: Option<Nutrient>,
    #[serde(rename = "VITK1")]
    pub vitk1: Option<Nutrient>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Digest {
    pub label: Option<String>,
    pub tag: Option<String>,
    pub schema_org_tag: Option<String>,
    pub total: Option<f64>,
    #[serde(rename = "hasRDI")]
    pub has_rdi: Option<bool>,
    pub daily: Option<f64>,
    pub unit: Option<String>,
    pub sub: Option<Vec<Sub>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Sub {
    pub label: Option<String>,
    pub tag: Option<String>,
    pub schema_org_tag: Option<String>,
    pub total: Option<f64>,
    #[serde(rename = "hasRDI")]
    pub has_rdi: Option<bool>,
    pub daily: Option<f64>,
    pub unit: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Links2 {
    #[serde(rename = "self")]
    pub self_field: Next,
}
