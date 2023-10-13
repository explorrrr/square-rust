//! Merchant

use serde::{Deserialize, Serialize};

use crate::api::models::enums::currency::Currency;

/// Represents a business that sells with Square.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Merchant {
    /// The Square-issued ID of the merchant.
    pub id: Option<String>,
    /// The name of the merchant's overall business.
    pub business_name: Option<String>,
    /// The country code associated with the merchant, in the two-letter format of ISO 3166. For example, US or JP.
    pub country: Country,
    /// The code indicating the [language preferences](https://developer.squareup.com/docs/build-basics/general-considerations/language-preferences) of the merchant, in [BCP 47 format](https://tools.ietf.org/html/bcp47#appendix-A). For example, en-US or fr-CA.
    pub language_code: Option<String>,
    /// The currency associated with the merchant, in ISO 4217 format. For example, the currency code for US dollars is USD.
    pub currency: Option<Currency>,
    /// The merchant's status.
    pub status: Option<MerchantStatus>,
    /// The ID of the main Location for this merchant.
    pub main_location_id: Option<String>,
    /// Read only The time when the merchant was created, in RFC 3339 format. For more information, see [Working with Dates](https://developer.squareup.com/docs/build-basics/working-with-dates).
    pub created_at: Option<String>,
}
