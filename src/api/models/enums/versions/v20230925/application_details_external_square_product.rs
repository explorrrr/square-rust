//! ApplicationDetailsExternalSquareProduct

use serde::{Deserialize, Serialize};

/// A list of products to return to external callers.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApplicationDetailsExternalSquareProcutV20230925 {
    Appointments,
    EcommerceApi,
    Invoices,
    OnlineStore,
    Other,
    Restaurants,
    Retail,
    SquarePos,
    TerminalApi,
    VirtualTerminal,
}
