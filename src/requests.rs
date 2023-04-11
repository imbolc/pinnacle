//! Typed Pinnacle API requests
use crate::{
    responses::*,
    traits::PinnacleApiRequest,
    util::{serialize_bool_1_or_skip, serialize_comma_separated_option},
};
use serde::{Deserialize, Serialize};

/// Returns current client balance.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClientBalance;

impl PinnacleApiRequest for GetClientBalance {
    const PATH: &'static str = "/v1/client/balance";
    type Response = ClientBalanceResponse;
}

/// Returns all sports leagues with the status whether they currently have lines or not
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSports;

impl PinnacleApiRequest for GetSports {
    const PATH: &'static str = "/v2/sports";
    type Response = SportsResponse;
}

/// Returns all sports leagues with the status whether they currently have lines or not
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLeagues {
    /// Sport id for which the leagues are requested.
    pub sport_id: i32,
}

impl PinnacleApiRequest for GetLeagues {
    const PATH: &'static str = "/v2/leagues";
    type Response = Leagues;
}

/// Returns all periods for a given sport.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPeriods {
    /// Sport id for which the leagues are requested.
    pub sport_id: i32,
}

impl PinnacleApiRequest for GetPeriods {
    const PATH: &'static str = "/v1/periods";
    type Response = SportPeriods;
}

/// Format to request the odds.
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

/// Returns straight odds for all non-settled events. Please note that it is  possible that the
/// event is in Get Fixtures response but not in Get Odds. This happens when the odds are not
/// currently available for wagering
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStraightOdds {
    /// The sportid for which to retrieve the odds.
    pub sport_id: i32,
    /// The leagueIds array may contain a list of comma separated league ids
    #[serde(serialize_with = "serialize_comma_separated_option")]
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
    #[serde(serialize_with = "serialize_bool_1_or_skip")]
    pub is_live: bool,
    /// Filter by EventIds
    #[serde(serialize_with = "serialize_comma_separated_option")]
    pub event_ids: Option<Vec<i64>>,
    /// 3 letter currency code as in the /currency response.
    /// Limits will be returned in the requested currency. Default is USD.
    pub to_currency_code: Option<String>,
}

impl PinnacleApiRequest for GetStraightOdds {
    const PATH: &'static str = "/v1/odds";
    type Response = OddsResponse;
}

/// Returns all **non-settled** events for the given sport. Please note that it is possible that
/// the event is in Get Fixtures response but not in Get Odds. This happens when the odds are not
/// currently available for wagering. Please note that it is possible to receive the same exact
/// response when using **since** parameter. This is rare and can be caused by internal updates of
/// event properties.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixtures {
    /// The ID of the sport to retrieve the fixtures for.
    pub sport_id: i32,
    /// An optional list of league IDs to filter the fixtures by.
    #[serde(serialize_with = "serialize_comma_separated_option")]
    pub league_ids: Option<Vec<i32>>,

    /// An optional flag indicating whether to retrieve only live events.
    pub is_live: Option<bool>,

    /// An optional timestamp to receive incremental updates.
    ///
    /// This is used to receive incremental updates. Use the value of last from previous fixtures
    /// response. When since parameter is not provided, the fixtures are delayed up to 1 minute to
    /// encourage the use of the parameter.
    pub since: Option<i64>,

    /// An optional list of event IDs to filter the fixtures by.
    #[serde(serialize_with = "serialize_comma_separated_option")]
    pub event_ids: Option<Vec<i32>>,
}

impl PinnacleApiRequest for GetFixtures {
    const PATH: &'static str = "/v1/fixtures";
    type Response = FixturesResponse;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straight_odds_request() {
        use serde_urlencoded::to_string as qs;

        assert_eq!(qs(GetStraightOdds::default()).unwrap(), "sportId=0");
        assert_eq!(
            qs(&GetStraightOdds {
                is_live: true,
                ..Default::default()
            })
            .unwrap(),
            "sportId=0&isLive=1"
        );
        assert_eq!(
            qs(&GetStraightOdds {
                league_ids: Some(vec![1, 2]),
                odds_format: Some(OddsFormat::Decimal),
                ..Default::default()
            })
            .unwrap(),
            "sportId=0&leagueIds=1%2C2&oddsFormat=Decimal"
        );
    }
}
