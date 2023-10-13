//! CustomField

use serde::{Deserialize, Serialize};

/// Describes a custom form field to add to the checkout page to collect more information from buyers during checkout.
///
/// For more information, see [Specify checkout options](https://developer.squareup.com/docs/checkout-api/optional-checkout-configurations#specify-checkout-options-1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldV20230925 {
    /// The title of the custom field.
    ///
    /// Min Length
    /// 1
    /// Max Length
    /// 50
    pub title: String,
}
