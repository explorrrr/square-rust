//! ArchivedState Enum


use serde::{Deserialize, Serialize};

/// Defines the values for the archived_state query expression used in [SearchCatalogItems](https://developer.squareup.com/reference/square/catalog-api/search-catalog-items) to return the archived, not archived or either type of catalog items.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArchivedState {
    /// Requested items are not archived with the is_archived attribute set to false.
    ArchivedStateNotArchived,
    /// Requested items are archived with the is_archived attribute set to true.
    ArchivedStateArchived,
    /// Requested items can be archived or not archived.
    ArchivedStateAll,
}
