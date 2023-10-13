//! FulfillmentFulfillmentLineItemApplication Enum

use serde::{Deserialize, Serialize};

/// The line_item_application describes what order line items this fulfillment applies to.
/// It can be ALL or ENTRY_LIST with a supplied list of fulfillment entries.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FulfillmentFulfillmentLineItemApplicationV20230925 {
    /// If ALL, entries must be unset.
    All,
    /// If ENTRY_LIST, supply a list of entries.
    EntryList,
}
