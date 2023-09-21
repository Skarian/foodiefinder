use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use reqwest_middleware::ClientWithMiddleware;
use std::time::Duration;

pub struct ReqwestClient(pub ClientWithMiddleware);

impl ReqwestClient {
    /// Creates a new `ReqwestClient` with default settings.
    ///
    /// # Returns
    ///
    /// A new `ReqwestClient` instance.
    ///
    /// # Examples
    /// ````
    /// let client = ReqwestClient::new();
    /// ````
    ///
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
            .build()
            .unwrap();

        let client_with_middleware = ClientBuilder::new(client)
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager::default(),
                options: None,
            }))
            .build();

        ReqwestClient(client_with_middleware)
    }
}
