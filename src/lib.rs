//! # pinnacle
//!
//! Rust Wrapper for Pinnacle Sports API
//!
//! The API is avalable at <https://pinnacleapi.github.io/>

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use std::result::Result as StdResult;

pub mod types;
mod util;

use types::*;

/// Pinnacle API client
#[derive(Debug)]
pub struct Client {
    username: String,
    password: String,
    client: reqwest::Client,
}

/// Errors
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum Error {
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

/// Result
pub type Result<T> = StdResult<T, Error>;

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
        let url = url.into_url()?;
        let text = self
            .client
            .get(url.clone())
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        if text.is_empty() {
            return Err(Error::EmptyJson(url));
        }
        util::parse_json(&text).map_err(|e| Error::DecodeJson(e, url))
    }

    /// Returns account balance
    pub async fn get_balance(&self) -> Result<ClientBalanceResponse> {
        self.get("https://api.pinnacle.com/v1/client/balance").await
    }

    /// Returns all sports with the status whether they currently have lines or not
    pub async fn get_sports(&self) -> Result<SportsResponse> {
        self.get("https://api.pinnacle.com/v2/sports").await
    }

    /// Returns all sports leagues with the status whether they currently have lines or not
    pub async fn get_sport_leagues(&self, sport_id: i32) -> Result<Leagues> {
        let url = format!("https://api.pinnacle.com/v2/leagues?sportId={sport_id}");
        self.get(url).await
    }

    /// Returns all periods for a given sport
    pub async fn get_sport_periods(&self, sport_id: i32) -> Result<SportPeriods> {
        let url = format!("https://api.pinnacle.com/v1/periods?sportId={sport_id}");
        self.get(url).await
    }

    /// Returns all periods for a given sport
    /// Usage: `client.get_straight_odds(&StraightOddsRequest {sport_id: 29, ..Default::default()}).await?`
    pub async fn get_straight_odds(&self, request: &StraightOddsRequest) -> Result<OddsResponse> {
        let qs = serde_urlencoded::to_string(request)
            .ok()
            .unwrap_or_default();
        let url = format!("https://api.pinnacle.com/v1/odds?{qs}");
        self.get(url).await
    }
}
