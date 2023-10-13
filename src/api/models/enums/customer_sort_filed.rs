//! CustomerSortField Enum

use serde::{Deserialize, Serialize};

/// Specifies customer attributes as the sort key to customer profiles returned from a search.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomerSortField {
    /// Use the default sort key. By default, customers are sorted alphanumerically by concatenating their given_name and family_name. If neither name field is set, string comparison is performed using one of the remaining fields in the following order: company_name, email, phone_number.
    Default,
    /// Use the creation date attribute (created_at) of customer profiles as the sort key.
    CreatedAt,
}
