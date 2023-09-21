use pyo3::prelude::*;

pub fn scrape_recipe_from_url(url: &str) -> PyResult<String> {
    Python::with_gil(|py| {
        let get_recipe_data = PyModule::from_code(
            py,
            r#"
import json
from recipe_scrapers import scrape_me

def scrape_url(url):
    results = scrape_me(url)
    json_results = results.to_json()
    json_string = json.dumps(json_results)
    return json_string
        "#,
            "getrecipes.py",
            "getrecipes",
        )?;

        // pass arguments as rust tuple
        let recipe_result: String = get_recipe_data
            .getattr("scrape_url")?
            .call1((url,))?
            .extract()?;

        Ok(recipe_result)
    })
}
