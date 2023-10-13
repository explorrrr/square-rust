//! PrePopulatedData

use serde::{Deserialize, Serialize};

use super::address::AddressV20230925;

/// Describes buyer data to prepopulate in the payment form.
///
/// For more information, see [Optional Checkout Configurations](https://developer.squareup.com/docs/checkout-api/optional-checkout-configurations).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrePopulatedDataV20230925 {
    /// The buyer email to prepopulate in the payment form.
    ///
    /// Max Length: 256
    buyer_email: Option<String>,
    /// The buyer phone number to prepopulate in the payment form.
    ///
    /// Max Length: 17
    buyer_phone_number: Option<String>,
    /// The buyer address to prepopulate in the payment form.
    buyer_address: Option<AddressV20230925>,
}
