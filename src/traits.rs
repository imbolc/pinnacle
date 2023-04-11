//! Traits
use async_trait::async_trait;
use reqwest::IntoUrl;
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;
use std::marker::{Send, Sync};

const API_ORIGIN: &str = "https://api.pinnacle.com";

/// Describes Pinnacle API request
pub trait PinnacleApiRequest {
    /// The API endpoint path
    const PATH: &'static str;

    /// The API response type
    type Response: DeserializeOwned + Serialize + Send;
}

/// API Client
#[async_trait]
pub trait PinnacleApiClient {
    /// The client error
    type Error: Error;

    /// General GET request using full URL
    async fn get_by_url<U, T>(&self, url: U) -> Result<T, Self::Error>
    where
        U: IntoUrl + Send,
        T: DeserializeOwned + Serialize + Send;

    /// Typed GET request
    async fn get<Q>(&self, query: &Q) -> Result<Q::Response, Self::Error>
    where
        Q: PinnacleApiRequest + Send + Serialize + Sync,
    {
        let qs = serde_urlencoded::to_string(query).ok().unwrap_or_default();
        let url = format!("{API_ORIGIN}{}?{qs}", Q::PATH);
        self.get_by_url(url).await
    }
}
