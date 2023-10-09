//! Customer details

/// Customer details
pub struct CustomerDetails {
    /// Indicates whether the customer initiated the payment.
    pub customer_initiated: Option<bool>,
    /// Inidicates that the seller keyed in payment details on behalf of the customer. This is used to flag payment as Mail Order/Telephone Order (MOTO).
    pub seller_keyed_in: Option<bool>,
}
