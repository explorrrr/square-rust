//! OrderFulfillmentFulfillmentLineItemApplication Enum

use serde::{Deserialize, Serialize};

/// The type of fulfillment. See [OrderFulfillmentType](#type-orderfulfillmenttype) for possible values.
/// It can be ALL or ENTRY_LIST with a supplied list of fulfillment entries.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderFulfillmentFulfillmentLineItemApplication {
    /// If ALL, entries must be unset.
    All,
    /// If ENTRY_LIST, supply a list of entries.
    EntryList,
}
