//! Pinnacle API client
use crate::{traits::PinnacleApiClient, util::parse_json};
use async_trait::async_trait;
use displaydoc::Display;
use reqwest::IntoUrl;
use serde::{de::DeserializeOwned, Serialize};
use std::marker::Send;
use thiserror::Error;

/// Pinnacle API client
#[derive(Debug)]
pub struct PinnacleClient {
    username: String,
    password: String,
    reqwest_client: reqwest::Client,
}

/// Errors
#[derive(Debug, Display, Error)]
pub enum PinnacleClientError {
    /// reqwest
    Reqwest(#[from] reqwest::Error),
    /// empty json from {0}
    EmptyJson(reqwest::Url),
    /// decode json from {1}
    DecodeJson(
        #[source] serde_path_to_error::Error<serde_json::Error>,
        reqwest::Url,
    ),
}

impl PinnacleClient {
    /// Creates a new client
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        let username = username.into();
        let password = password.into();
        let reqwest_client = reqwest::Client::new();
        Self {
            username,
            password,
            reqwest_client,
        }
    }
}

#[async_trait]
impl PinnacleApiClient for PinnacleClient {
    type Error = PinnacleClientError;

    async fn get_by_url<U, T>(&self, url: U) -> Result<T, Self::Error>
    where
        U: IntoUrl + Send,
        T: DeserializeOwned + Serialize + Send,
    {
        let url = url.into_url()?;
        println!("GET {url}");
        let text = self
            .reqwest_client
            .get(url.clone())
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        if text.is_empty() {
            return Err(PinnacleClientError::EmptyJson(url));
        }
        parse_json(&text).map_err(|e| PinnacleClientError::DecodeJson(e, url))
    }
}
