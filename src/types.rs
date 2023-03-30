//! Typed responses
use crate::util;
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

/// Format in which we return the odds
#[derive(Debug, Deserialize, Serialize)]
pub enum OddsFormat {
    /// American
    American,
    /// Decimal
    Decimal,
    /// HongKong
    HongKong,
    /// Indonesian
    Indonesian,
    /// Malay
    Malay,
}

/// Straight Odds query params
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StraightOddsOptions {
    /// The leagueIds array may contain a list of comma separated league ids
    #[serde(serialize_with = "util::serialize_comma_separated_option")]
    pub league_ids: Option<Vec<i32>>,
    /// Format in which we return the odds. Default is American.
    pub odds_format: Option<OddsFormat>,
    /// This is used to receive incremental updates. Use the value of last from previous odds
    /// response. When since parameter is not provided, the odds are delayed up to 1 min to
    /// encourage the use of the parameter. Please note that when using since parameter you will
    /// get in the response ONLY changed periods. If a period did not have any changes it will
    /// not be in the response.
    pub since: Option<i64>,
    /// To retrieve ONLY live odds set the value to 1 (isLive=1).
    /// Otherwise response will have all odds.
    #[serde(serialize_with = "util::serialize_bool_1_or_skip")]
    pub is_live: bool,
    /// Filter by EventIds
    #[serde(serialize_with = "util::serialize_comma_separated_option")]
    pub event_ids: Option<Vec<i64>>,
    /// 3 letter currency code as in the /currency response.
    /// Limits will be returned in the requested currency. Default is USD.
    pub to_currency_code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straight_odds_query() {
        use serde_urlencoded::to_string as qs;

        assert_eq!(qs(&StraightOddsOptions::default()).unwrap(), "");
        assert_eq!(
            qs(&StraightOddsOptions {
                is_live: true,
                ..Default::default()
            })
            .unwrap(),
            "isLive=1"
        );
        assert_eq!(
            qs(&StraightOddsOptions {
                league_ids: Some(vec![1, 2]),
                odds_format: Some(OddsFormat::Decimal),
                ..Default::default()
            })
            .unwrap(),
            "leagueIds=1%2C2&oddsFormat=Decimal"
        );
    }
}
