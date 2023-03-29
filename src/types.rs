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
