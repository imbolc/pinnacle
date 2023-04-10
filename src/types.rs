//! Typed responses
use crate::util;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the balance details of a client.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientBalanceResponse {
    /// The amount available for betting.
    pub available_balance: f64,
    /// The sum of not yet settled bet amounts.
    pub outstanding_transactions: f64,
    /// The client's credit.
    pub given_credit: f64,
    /// The client's currency code.
    pub currency: String,
}

/// Represents a sports response containing a list of sports.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SportsResponse {
    /// The list of sports.
    pub sports: Vec<Sport>,
}

/// Represents a sport.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sport {
    /// The sport ID.
    pub id: i32,
    /// The sport name.
    pub name: String,
    /// Whether the sport currently has events or specials.
    pub has_offerings: bool,
    /// Indicates how many specials are in the given sport.
    pub league_specials_count: i32,
    /// Indicates how many event specials are in the given sport.
    pub event_specials_count: i32,
    /// Indicates how many events are in the given sport.
    pub event_count: i32,
}

/// Represents a leagues response containing a list of leagues.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leagues {
    /// The list of leagues.
    pub leagues: Vec<League>,
}

/// Represents a league.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct League {
    /// The league ID.
    pub id: i32,
    /// The name of the league.
    pub name: String,
    /// Specifies whether the home team is team1 or team2. You need this information to place a bet.
    pub home_team_type: String,
    /// Whether the league currently has events or specials.
    pub has_offerings: bool,
    /// Represents grouping for the league, usually a region/country.
    pub container: String,
    /// Specifies whether you can place parlay round robins on events in this league.
    pub allow_round_robins: bool,
    /// Indicates how many specials are in the given league.
    pub league_specials_count: i32,
    /// Indicates how many game specials are in the given league.
    pub event_specials_count: i32,
    /// Indicates how many events are in the given league.
    pub event_count: i32,
}

/// Represents a period for a sport.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SportPeriod {
    /// The period number.
    pub number: i32,
    /// Description for the period.
    pub description: String,
    /// Short description for the period.
    pub short_description: String,
    /// Description for the spread.
    pub spread_description: String,
    /// Description for the moneyline.
    pub moneyline_description: String,
    /// Description for the totals.
    pub total_description: String,
    /// Description for team1 totals.
    pub team1_total_description: String,
    /// Description for team2 totals.
    pub team2_total_description: String,
    /// Short description for the spread.
    pub spread_short_description: String,
    /// Short description for the moneyline.
    pub moneyline_short_description: String,
    /// Short description for the totals.
    pub total_short_description: String,
    /// Short description for team1 totals.
    pub team1_total_short_description: String,
    /// Short description for team2 totals.
    pub team2_total_short_description: String,
}

/// Periods
#[derive(Debug, Deserialize, Serialize)]
pub struct SportPeriods {
    /// Periods
    pub periods: Vec<SportPeriod>,
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

/// Straight Odds request params
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StraightOddsRequest {
    /// The sportid for which to retrieve the odds.
    pub sport_id: i32,
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

/// Odds Response
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsResponse {
    /// Same as requested sport Id.
    pub sport_id: i32,
    /// Use this value for the subsequent requests for since query parameter to get just the
    /// changes since the previous response.
    pub last: i64,
    /// Contains a list of Leagues.
    pub leagues: Vec<OddsLeague>,
}

/// Odds League
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsLeague {
    /// League Id.
    pub id: i32,
    /// Contains a list of events.
    pub events: Vec<OddsEvent>,
}

/// Odds Event
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsEvent {
    /// Event Id.
    pub id: i64,
    /// Away team score. Only for live soccer events. Supported only for full match period
    /// (number=0).
    pub away_score: Option<f64>,
    /// Home team score. Only for live soccer events. Supported only for full match period
    /// (number=0).
    pub home_score: Option<f64>,
    /// Away team red cards. Only for live soccer events. Supported only for full match period
    /// (number=0).
    pub away_red_cards: Option<i32>,
    /// Home team red cards. Only for live soccer events. Supported only for full match period
    /// (number=0).
    pub home_red_cards: Option<i32>,
    /// Contains a list of periods.
    pub periods: Vec<OddsPeriod>,
}

/// Odds Period
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsPeriod {
    /// Line Id.
    pub line_id: i64,
    /// This represents the period of the match.
    pub number: i32,
    /// Period’s wagering cut-off date in UTC.
    pub cutoff: DateTime<Utc>,
    /// 1 - online, period is open for betting. 2 - offline, period is not open for betting.
    pub status: i32,
    /// Maximum spread bet volume. See [How to calculate max risk from the max volume](https://github.com/pinnacleapi/pinnacleapi-documentation/blob/master/FAQ.md#how-to-calculate-max-risk-from-the-max-volume-limits-in-odds)
    pub max_spread: Option<f64>,
    /// Maximum moneyline bet volume. See [How to calculate max risk from the max volume](https://github.com/pinnacleapi/pinnacleapi-documentation/blob/master/FAQ.md#how-to-calculate-max-risk-from-the-max-volume-limits-in-odds)
    pub max_moneyline: Option<f64>,
    /// Maximum total points bet volume. See [How to calculate max risk from the max volume](https://github.com/pinnacleapi/pinnacleapi-documentation/blob/master/FAQ.md#how-to-calculate-max-risk-from-the-max-volume-limits-in-odds)
    pub max_total: Option<f64>,
    /// Maximum team total points bet volume. See [How to calculate max risk from the max volume](https://github.com/pinnacleapi/pinnacleapi-documentation/blob/master/FAQ.md#how-to-calculate-max-risk-from-the-max-volume-limits-in-odds)
    pub max_team_total: Option<f64>,
    /// Date time of the last moneyline update.
    pub moneyline_updated_at: Option<DateTime<Utc>>,
    /// Date time of the last spread update.
    pub spread_updated_at: Option<DateTime<Utc>>,
    /// Date time of the last total update.
    pub total_updated_at: Option<DateTime<Utc>>,
    /// Date time of the last team total update.
    pub team_total_updated_at: Option<DateTime<Utc>>,
    /// Container for spread odds.
    pub spreads: Option<Vec<OddsSpread>>,
    /// Moneyline odds.
    pub moneyline: Option<OddsMoneyline>,
    /// Container for team total points.
    pub totals: Option<Vec<OddsTotal>>,
    /// Team total points odds.
    pub team_total: Option<OddsTeamTotals>,
    /// Period away team score. Only for live soccer events. Supported only for Match (number=0)
    /// and Extra Time (number=3).
    pub away_score: Option<f64>,
    /// Period home team score. Only for live soccer events. Supported only for Match (number=0)
    /// and Extra Time (number=3).
    pub home_score: Option<f64>,
    /// Period away team red cards. Only for live soccer events. Supported only for Match
    /// (number=0) and Extra Time (number=3).
    pub away_red_cards: Option<i32>,
    /// Period home team red cards. Only for live soccer events. Supported only for Match
    /// (number=0) and Extra Time (number=3).
    pub home_red_cards: Option<i32>,
}

/// Odds Spread
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsSpread {
    /// This is present only if it's an alternative line.
    pub alt_line_id: Option<i64>,
    /// Home team handicap.
    pub hdp: f64,
    /// Home team price.
    pub home: f64,
    /// Away team price.
    pub away: f64,
    /// Maximum bet volume. Present only on alternative lines, if set it overides `maxSpread`
    /// market limit.
    pub max: Option<f64>,
}

/// Odds Moneyline
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsMoneyline {
    /// Away team price.
    pub home: f64,
    /// Away team price.
    pub away: f64,
    /// Draw price. This is present only for events we offer price for draw.
    pub draw: Option<f64>,
}

/// Odds Total
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsTotal {
    /// This is present only if it’s an alternative line.
    pub alt_line_id: Option<i64>,
    /// Total points.
    pub points: f64,
    /// Over price.
    pub over: f64,
    /// Under price.
    pub under: f64,
    /// Maximum bet volume. Present only on alternative lines, if set it overrides `maxTotal`
    /// market limit.
    pub max: Option<f64>,
}

/// Odds Team Totals
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsTeamTotals {
    /// Home team total points, over and under prices.
    pub home: Option<OddsTeamTotal>,
    /// Away team total points, over and under prices.
    pub away: Option<OddsTeamTotal>,
}

/// Odds Team Total
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsTeamTotal {
    /// Total points.
    pub points: f64,
    /// Over price.
    pub over: f64,
    /// Under price.
    pub under: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straight_odds_query() {
        use serde_urlencoded::to_string as qs;

        assert_eq!(qs(&StraightOddsRequest::default()).unwrap(), "sportId=0");
        assert_eq!(
            qs(&StraightOddsRequest {
                is_live: true,
                ..Default::default()
            })
            .unwrap(),
            "sportId=0&isLive=1"
        );
        assert_eq!(
            qs(&StraightOddsRequest {
                league_ids: Some(vec![1, 2]),
                odds_format: Some(OddsFormat::Decimal),
                ..Default::default()
            })
            .unwrap(),
            "sportId=0&leagueIds=1%2C2&oddsFormat=Decimal"
        );
    }
}
