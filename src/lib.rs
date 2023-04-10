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
//! #[tokio::main]
//! async fn main() {
//!    let client = PinnacleClient::new("pinnacle_user", "pinnacle_password");
//!    let req = GetStraightOdds {
//!        sport_id: 29,
//!        ..Default::default()
//!    };
//!    let resp = client.get(&req).await.unwrap();
//!    dbg!(resp);
//! }
//! ```
//!
//! [api]: https://pinnacleapi.github.io/

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

pub mod client;
pub mod prelude;
pub mod requests;
pub mod responses;
pub mod traits;
pub mod util;
