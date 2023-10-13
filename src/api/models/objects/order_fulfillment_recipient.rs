//! OrderFulfillmentRecipient

use serde::{Deserialize, Serialize};

/// Information about the fulfillment recipient.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFulfillmentRecipient {
    /// The ID of the customer associated with the fulfillment. If customer_id is provided, the fulfillment recipient's display_name, email_address, and phone_number are automatically populated from the targeted customer profile. If these fields are set in the request, the request values override the information from the customer profile. If the targeted customer profile does not contain the necessary information and these fields are left unset, the request results in an error.
    ///
    /// Max Length: 191
    pub customer_id: Option<String>,
    /// The display name of the fulfillment recipient. This field is required. If provided, the display name overrides the corresponding customer profile value indicated by customer_id.
    ///
    /// Max Length: 255
    pub display_name: Option<String>,
    /// The email address of the fulfillment recipient. If provided, the email address overrides the corresponding customer profile value indicated by customer_id.
    ///
    /// Max Length: 255
    pub email_address: Option<String>,
    /// The phone number of the fulfillment recipient. This field is required. If provided, the phone number overrides the corresponding customer profile value indicated by customer_id.
    ///
    /// Max Length: 17
    pub phone_number: Option<String>,
    /// The address of the fulfillment recipient. This field is required. If provided, the address overrides the corresponding customer profile value indicated by customer_id.
    pub address: Option<Address>,
}
