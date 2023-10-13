//! Checkout

use serde::{Deserialize, Serialize};

/// Square Checkout lets merchants accept online payments for supported payment types using a checkout workflow hosted on squareup.com.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkout {
    /// ID generated by Square Checkout when a new checkout is requested.
    pub id: Option<String>,
    /// The URL that the buyer's browser should be redirected to after the checkout is completed.
    pub checkout_page_url: Option<String>,
    /// If true, Square Checkout will collect shipping information on your behalf and store that information with the transaction information in your Square Dashboard.
    ///
    /// Default: false.
    pub ask_for_shipping_address: Option<bool>,
    /// The email address to display on the Square Checkout confirmation page and confirmation email that the buyer can use to contact the merchant.
    ///
    /// If this value is not set, the confirmation page and email will display the primary email address associated with the merchant's Square account.
    ///
    /// Default: none; only exists if explicitly set.
    pub merchant_support_email: Option<String>,
    /// If provided, the buyer's email is pre-populated on the checkout page as an editable text field.
    ///
    /// Default: none; only exists if explicitly set.
    pub pre_populate_buyer_email: Option<String>,
    /// If provided, the buyer's shipping info is pre-populated on the checkout page as editable text fields.
    ///
    /// Default: none; only exists if explicitly set.
    pub pre_populate_shipping_address: Option<Address>,
    /// The URL to redirect to after checkout is completed with checkoutId, Square's orderId, transactionId, and referenceId appended as URL parameters. For example, if the provided redirect_url is http://www.example.com/order-complete, a successful transaction redirects the customer to:
    ///
    /// <pre><code>http://www.example.com/order-complete?checkoutId=xxxxxx&orderId=xxxxxx&referenceId=xxxxxx&transactionId=xxxxxx</code></pre>
    ///
    /// If you do not provide a redirect URL, Square Checkout will display an order confirmation page on your behalf; however Square strongly recommends that you provide a redirect URL so you can verify the transaction results and finalize the order through your existing/normal confirmation workflow.
    pub redirect_url: Option<String>,
    /// Order to be checked out.
    pub order: Option<Order>,
    /// Read only The time when the checkout was created, in RFC 3339 format.
    pub created_at: Option<String>,
}