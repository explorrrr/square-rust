//! Customer

use serde::{Deserialize, Serialize};

use crate::api::models::enums::customer_creation_source::CustomerCreationSource;
use crate::api::models::objects::address::Address;
use crate::api::models::objects::customer_preferences::CustomerPreferences;
use crate::api::models::objects::customer_tax_ids::CustomerTaxIds;

/// Represents a Square customer profile, which can have one or more cards on file associated with it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    /// A unique Square-assigned ID for the customer profile.
    ///
    /// If you need this ID for an API request, use the ID returned when you created the customer profile or call the [SearchCustomers](https://developer.squareup.com/reference/square/customers-api/search-customers) or [ListCustomers](https://developer.squareup.com/reference/square/customers-api/list-customers) endpoint.
    pub id: Option<String>,
    /// Read only The timestamp when the customer profile was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// Read only The timestamp when the customer profile was last updated, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
    /// The given name (that is, the first name) associated with the customer profile.
    pub given_name: Option<String>,
    /// The family name (that is, the last name) associated with the customer profile.
    pub family_name: Option<String>,
    /// A nickname for the customer profile.
    pub nickname: Option<String>,
    /// A business name associated with the customer profile.
    pub company_name: Option<String>,
    /// The email address associated with the customer profile.
    pub email_address: Option<String>,
    /// The physical address associated with the customer profile.
    pub address: Option<Address>,
    /// The phone number associated with the customer profile.
    pub phone_number: Option<String>,
    /// The birthday associated with the customer profile, in YYYY-MM-DD format. For example, 1998-09-21 represents September 21, 1998, and 0000-09-21 represents September 21 (without a birth year).
    pub birthday: Option<String>,
    /// An optional, second ID used to associate the customer profile with an entity in another system.
    pub reference_id: Option<String>,
    /// A custom note associated with the customer profile.
    pub note: Option<String>,
    /// Represents general customer preferences.
    pub preferences: Option<CustomerPreferences>,
    /// The method used to create the customer profile.
    pub creation_source: Option<CustomerCreationSource>,
    /// The IDs of [customer groups](https://developer.squareup.com/reference/square/objects/CustomerGroup) the customer belongs to.
    pub group_ids: Option<Vec<String>>,
    /// The IDs of [customer segments](https://developer.squareup.com/reference/square/objects/CustomerSegment) the customer belongs to.
    pub segment_ids: Option<Vec<String>>,
    /// The Square-assigned version number of the customer profile. The version number is incremented each time an update is committed to the customer profile, except for changes to customer segment membership and cards on file.
    pub version: Option<i64>,
    /// The tax ID associated with the customer profile. This field is present only for customers of sellers in EU countries or the United Kingdom. For more information, see [Customer tax IDs](https://developer.squareup.com/docs/customers-api/what-it-does#customer-tax-ids).
    pub tax_ids: Option<Vec<CustomerTaxIds>>,
}
