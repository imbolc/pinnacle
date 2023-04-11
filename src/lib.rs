//! # pinnacle
//!
//! Rust Wrapper for [Pinnacle Sports API][api]
//!
//! > **Note**
//! > Not all of the API is currently wrapped, but it should be relatively easy to add missing
//! > endpoints. All you need to do is implement the corresponding
//! > [request](`traits::PinnacleApiRequest`) and probably a [response](`responses`).
//! > Don't hesitate to make a PR if you do.
//!
//! Here are all the currently wrapped [`requests`].
//!
//! ## Usage
//!
//! ```rust,no_run
//! use pinnacle::prelude::*;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), PinnacleClientError> {
//!    let client = PinnacleClient::new("pinnacle_user", "pinnacle_password");
//!    let req = GetStraightOdds {
//!        sport_id: 29,
//!        ..Default::default()
//!    };
//!    let odds = client.get(&req).await?;
//! #  Ok(())
//! # }
//! ```
//!
//! You can also use a client that caches responses, which is helpful for development purposes:
//!
//! ```rust,no_run
//! use pinnacle::prelude::*;
//! use std::time::Duration;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), PinnacleClientError> {
//!    let client = PinnacleCachingClient::new(
//!        "pinnacle_user",
//!        "pinnacle_password",
//!        "cache-folder",
//!        Duration::from_secs(60 * 5),
//!        );
//!    let balance = client.get(&GetClientBalance).await?;
//!    // Now, if you repeat the request within 5 minutes, the cached version will be used
//!    // instead of making a new request.
//!    let cached_balance = client.get(&GetClientBalance).await?;
//! #  Ok(())
//! # }
//! ```
//!
//! [api]: https://pinnacleapi.github.io/

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

pub mod caching_client;
pub mod client;
pub mod prelude;
pub mod requests;
pub mod responses;
pub mod traits;
pub mod util;
