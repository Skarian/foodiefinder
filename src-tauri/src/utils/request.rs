use reqwest_middleware::RequestBuilder;
use std::collections::HashMap;

use crate::error::RequestError;
use crate::types::client::ReqwestClient;

/// Builds a GET request with the provided URL and query parameters.
///
/// The function will return a `RequestBuilder` that can be used to further customize the request,like adding headers or setting the request body. If locking the client fails, the function will return a `RequestError::PoisonedLock` error.
///
/// * `url`: The URL for the request. This should be a valid HTTP or HTTPS URL.
/// * `params`: An optional map of query parameters to add to the request. If this is `None`,
///   no query parameters will be added.
/// * `client`: A shared Reqwest client wrapped in a Tauri-managed state. This is used to
///   actually build the request.

pub async fn build_request(
    url: &str,
    params: Option<HashMap<&str, &str>>,
    client: tauri::State<'_, ReqwestClient>,
) -> Result<RequestBuilder, RequestError> {
    let request_builder = client.0.get(url);
    match params {
        Some(p) => Ok(request_builder.query(&p)),
        None => Ok(request_builder),
    }
}
