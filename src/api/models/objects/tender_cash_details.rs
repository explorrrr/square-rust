//! TenderCashDetails

use serde::{Deserialize, Serialize};

/// Represents the details of a tender with type CASH.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenderCashDetails {
    /// The total amount of cash provided by the buyer, before change is given.
    pub buyer_tendered_money: Option<Money>,
    /// The amount of change returned to the buyer.
    pub change_back_money: Option<Money>,
}
