//! # pinnacle
//!
//! Rust Wrapper for Pinnacle Sports API
//!
//! The API is avalable at <https://pinnacleapi.github.io/>

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

use reqwest::IntoUrl;
pub use reqwest::{Error, Result};
use serde::de::DeserializeOwned;

mod types;
pub use types::Balance;

const BALANCE_URL: &str = "https://api.pinnacle.com/v1/client/balance";

/// Pinnacle API client
#[derive(Debug)]
pub struct Client {
    username: String,
    password: String,
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    /// Creates a new client
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        let username = username.into();
        let password = password.into();
        let base_url = "https://api.pinnacle.com/".into();
        let client = reqwest::Client::new();
        Self {
            username,
            password,
            base_url,
            client,
        }
    }

    /// Changes the api base url
    pub fn with_base_url(&mut self, url: impl Into<String>) {
        self.base_url = url.into();
    }

    /// A general GET request
    pub async fn get<U: IntoUrl, T: DeserializeOwned>(&self, url: U) -> Result<T> {
        self.client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    /// Returns account balance
    pub async fn get_balance(&self) -> Result<Balance> {
        self.get(BALANCE_URL).await
    }
}
