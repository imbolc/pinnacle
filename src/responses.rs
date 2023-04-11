//! Typed Pinnacle API responses
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the balance details of a client.
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SportsResponse {
    /// The list of sports.
    pub sports: Vec<Sport>,
}

/// Represents a sport.
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Leagues {
    /// The list of leagues.
    pub leagues: Vec<League>,
}

/// Represents a league.
#[derive(Debug, Deserialize, Serialize)]
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

/// Periods
#[derive(Debug, Deserialize, Serialize)]
pub struct SportPeriods {
    /// Periods
    pub periods: Vec<SportPeriod>,
}

/// Represents a period for a sport.
#[derive(Debug, Deserialize, Serialize)]
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

/// Odds Response
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsLeague {
    /// League Id.
    pub id: i32,
    /// Contains a list of events.
    pub events: Vec<OddsEvent>,
}

/// Odds Event
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsTeamTotals {
    /// Home team total points, over and under prices.
    pub home: Option<OddsTeamTotal>,
    /// Away team total points, over and under prices.
    pub away: Option<OddsTeamTotal>,
}

/// Odds Team Total
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OddsTeamTotal {
    /// Total points.
    pub points: f64,
    /// Over price.
    pub over: f64,
    /// Under price.
    pub under: f64,
}

/// Represents the response from the /v1/fixtures endpoint.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FixturesResponse {
    /// Same as requested sport ID.
    pub sport_id: i32,
    /// Use this value for the subsequent requests for since query parameter to get just the changes
    /// since previous response.
    pub last: i64,
    /// Contains a list of leagues.
    pub league: Vec<FixturesLeague>,
}

/// Represents a league in the fixtures response.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FixturesLeague {
    /// League ID.
    pub id: i32,

    /// League name.
    pub name: String,

    /// Contains a list of events.
    pub events: Vec<Fixture>,
}

/// Fixture object
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Fixture {
    /// Event id.
    pub id: i64,
    /// If event is linked to another event, parentId will be populated.
    /// Live event would have pre game event as parent id.
    pub parent_id: Option<i64>,
    /// Start time of the event in UTC.
    pub starts: String,
    /// Home team name.
    pub home: String,
    /// Away team name.
    pub away: String,
    /// Team1 rotation number. Please note that in the next version of /fixtures,
    /// rotNum property will be decommissioned. ParentId can be used instead to group the related events.
    pub rot_num: Option<String>,
    /// Indicates live status of the event.
    /// - 0 = No live betting will be offered on this event.
    /// - 1 = Live betting event.
    /// - 2 = Live betting will be offered on this match, but on a different event.
    /// Please note that pre-game and live events are different.
    pub live_status: i32,
    /// Home team pitcher. Only for Baseball.
    pub home_pitcher: Option<String>,
    /// Away team pitcher. Only for Baseball.
    pub away_pitcher: Option<String>,
    /// This is deprecated parameter, please check period's `status` in the `/odds` endpoint to see if it's open for betting.
    /// - O = This is the starting status of a game.
    /// - H = This status indicates that the lines are temporarily unavailable for betting.
    /// - I = This status indicates that one or more lines have a red circle (lower maximum bet amount).
    pub status: Option<String>,
    /// Parlay status of the event.
    /// - 0 = Allowed to parlay, without restrictions.
    /// - 1 = Not allowed to parlay this event.
    /// - 2 = Allowed to parlay with the restrictions. You cannot have more than one leg from the same event in the parlay.
    /// All events with the same rotation number are treated as same event.
    pub parlay_restriction: i32,
    /// Whether an event is offer with alternative teaser points. Events with alternative teaser points may vary from teaser definition.
    pub alt_teaser: bool,
    /// Specifies based on what the event will be resulted, e.g. Corners, Bookings.
    pub resulting_unit: Option<String>,
    /// Fixture version, goes up when there is a change in the fixture.
    pub version: i64,
}
