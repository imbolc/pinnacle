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

/// Sports
#[derive(Debug, Deserialize, Serialize)]
pub struct Sports {
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

/// Leagues
#[derive(Debug, Deserialize, Serialize)]
pub struct Leagues {
    /// Leagues
    pub leagues: Vec<League>,
}

/// Period
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    /// Period Number
    pub number: i32,
    /// Description for the period
    pub description: String,
    /// Short description for the period
    pub short_description: String,
    /// Description for the Spread
    pub spread_description: String,
    /// Description for the Moneyline
    pub moneyline_description: String,
    /// Description for the Totals
    pub total_description: String,
    /// Description for Team1 Totals
    pub team1_total_description: String,
    /// Description for Team2 Totals
    pub team2_total_description: String,
    /// Short description for the Spread
    pub spread_short_description: String,
    /// Short description for the Moneyline
    pub moneyline_short_description: String,
    /// Short description for the Totals
    pub total_short_description: String,
    /// Short description for Team1 Totals
    pub team1_total_short_description: String,
    /// Short description for Team2 Totals
    pub team2_total_short_description: String,
}

/// Periods
#[derive(Debug, Deserialize, Serialize)]
pub struct Periods {
    /// Periods
    pub periods: Vec<Period>,
}
