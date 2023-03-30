//! # pinnacle
//!
//! Rust Wrapper for Pinnacle Sports API
//!
//! The API is avalable at <https://pinnacleapi.github.io/>

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

use reqwest::IntoUrl;
pub use reqwest::{Error, Result};
use serde::de::DeserializeOwned;

pub mod types;

/// Pinnacle API client
#[derive(Debug)]
pub struct Client {
    username: String,
    password: String,
    client: reqwest::Client,
}

impl Client {
    /// Creates a new client
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        let username = username.into();
        let password = password.into();
        let client = reqwest::Client::new();
        Self {
            username,
            password,
            client,
        }
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
    pub async fn get_balance(&self) -> Result<types::Balance> {
        self.get("https://api.pinnacle.com/v1/client/balance").await
    }

    /// Returns all sports with the status whether they currently have lines or not
    pub async fn get_sports(&self) -> Result<Vec<types::Sport>> {
        let types::SportsContainer { sports } =
            self.get("https://api.pinnacle.com/v2/sports").await?;
        Ok(sports)
    }

    /// Returns all sports with the status whether they currently have lines or not
    pub async fn get_sport_leagues(&self, sport_id: i32) -> Result<Vec<types::League>> {
        let url = format!("https://api.pinnacle.com/v2/leagues?sportId={sport_id}");
        let types::LeaguesContainer { leagues } = self.get(url).await?;
        Ok(leagues)
    }
}
