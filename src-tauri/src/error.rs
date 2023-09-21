use std::io;
use url::ParseError;

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("Failed to make a request: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Failed to serialize the response: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Failed to execute middleware: {0}")]
    Middleware(#[from] reqwest_middleware::Error),
    #[error("Error parsing LD+JSON info for Recipe: {0}")]
    RecipeJson(#[from] RecipeJsonError),
    #[error("Failed to set valid status on hits: {0}")]
    SetAllHitsValid(#[from] SetAllHitsValidError),
    #[error("Failed to scrape recipe via PyO3 and recipe-scrapers: {0}")]
    PyO3(#[from] pyo3::PyErr),
}

impl serde::Serialize for RequestError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecipeJsonError {
    #[error("HTML parsing error")]
    HtmlParsing,
    #[error("JSON parsing error: {0}")]
    JsonParsing(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CheckScrapableError {
    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] ParseError),
    #[error("No host in the URL")]
    NoHost,
    #[error("Failed to extract domain from host")]
    NoDomain,
}

#[derive(Debug, thiserror::Error)]
pub enum GetHitBodiesError {
    #[error("Middleware error: {0}")]
    MiddlewareError(#[from] reqwest_middleware::Error),
    #[error("HTTP request/response error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SetAllHitsValidError {
    #[error("Failed to get hit bodies")]
    GetHitBodiesFailed,
    #[error("Failed to set status")]
    SetStatusFailed,
    #[error("Mutex was poisoned")]
    MutexPoisoned,
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigPathError {
    #[error("Failed while creating a new custom config directory, none existed")]
    CreateAppConfigDir(#[from] io::Error),
    #[error("Tauri unable to locate app data dir")]
    HomeDirNotFound,
}

#[derive(thiserror::Error, Debug)]
pub enum DBError {
    #[error("Config directory error: {0}")]
    ConfigPath(#[from] ConfigPathError),
    #[error("Failed to connect to db: {0}")]
    Connection(#[from] rusqlite::Error),
    #[error("Unable to serialize/deserialize JSON: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Missing id field in RecipeDetails, likely in updating values")]
    MissingID,
    #[error("Error managing connection pool")]
    ConnectionPool,
    #[error("Record not found")]
    NoRecord,
    #[error("Error making a request: {0}")]
    Request(#[from] RequestError),
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
}

impl serde::Serialize for DBError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
