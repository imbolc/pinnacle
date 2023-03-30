//! Typed responses
use serde::{Deserialize, Serialize};

/// Account balance
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    /// Amount available for betting
    pub available_balance: f64,
    /// Sum of not yet settled bet amounts
    pub outstanding_transactions: f64,
    /// Client’s credit
    pub given_credit: f64,
    /// Client’s currency code
    pub currency: String,
}

/// Sport
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sport {
    /// Sport Id
    pub id: i32,
    /// Sport name
    pub name: String,
    /// Whether the sport currently has events or specials
    pub has_offerings: bool,
    /// Indicates how many specials are in the given sport
    pub league_specials_count: i32,
    /// Indicates how many event specials are in the given sport
    pub event_specials_count: i32,
    /// Indicates how many events are in the given sport
    pub event_count: i32,
}

/// Wrapper for the sports response
#[derive(Debug, Deserialize, Serialize)]
pub struct SportsContainer {
    /// Sports
    pub sports: Vec<Sport>,
}

/// League
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct League {
    /// League Id
    pub id: i32,
    /// Name of the league
    pub name: String,
    /// Specifies whether the home team is team1 or team2. You need this information to place a bet.
    pub home_team_type: String,
    /// Whether the league currently has events or specials
    pub has_offerings: bool,
    /// Represents grouping for the league, usually a region/country
    pub container: String,
    /// Specifies whether you can place parlay round robins on events in this league
    pub allow_round_robins: bool,
    /// Indicates how many specials are in the given league
    pub league_specials_count: i32,
    /// Indicates how many game specials are in the given league
    pub event_specials_count: i32,
    /// Indicates how many events are in the given league
    pub event_count: i32,
}

/// Wrapper for the leagues response
#[derive(Debug, Deserialize, Serialize)]
pub struct LeaguesContainer {
    /// Leagues
    pub leagues: Vec<League>,
}
