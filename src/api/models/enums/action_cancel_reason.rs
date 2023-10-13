//! ActionCancelReason Enum

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActionCancelReason {
    /// A person canceled the TerminalCheckout from a Square device.
    BuyerCanceled,
    /// A client canceled the TerminalCheckout using the API.
    SellerCanceled,
    /// The TerminalCheckout timed out (see deadline_duration on the TerminalCheckout).
    TimedOut,
}
