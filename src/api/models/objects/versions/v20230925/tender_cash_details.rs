//! TenderCashDetails

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;

/// Represents the details of a tender with type CASH.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenderCashDetailsV20230925 {
    /// The total amount of cash provided by the buyer, before change is given.
    pub buyer_tendered_money: Option<MoneyV20230925>,
    /// The amount of change returned to the buyer.
    pub change_back_money: Option<MoneyV20230925>,
}
