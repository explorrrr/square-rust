//! CustomerTaxIds

use serde::{Deserialize, Serialize};

/// The customer's tax IDs.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerTaxIdsV20230925 {
    /// The EU VAT identification number for the customer. For example, IE3426675K. The ID can contain alphanumeric characters only.
    /// Max Length: 20
    pub eu_vat: Option<String>,
}
