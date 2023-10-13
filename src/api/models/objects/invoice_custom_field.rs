//! InvoiceCustomField

use serde::{Deserialize, Serialize};

/// An additional seller-defined and customer-facing field to include on the invoice.
///
/// For more information, see [Custom fields](https://developer.squareup.com/docs/invoices-api/overview#custom-fields).
///
/// Adding custom fields to an invoice requires an [Invoices Plus subscription](https://developer.squareup.com/docs/invoices-api/overview#invoices-plus-subscription).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceCustomField {
    /// The label or title of the custom field.
    ///
    /// This field is required for a custom field.
    ///
    /// Max Length
    /// 30
    pub label: Option<String>,
    /// The text of the custom field.
    ///
    /// If omitted, only the label is rendered.
    ///
    /// Max Length
    /// 2000
    pub value: Option<String>,
    /// The location of the custom field on the invoice.
    ///
    /// This field is required for a custom field.
    pub placement: Option<InvoiceCustomFieldPlacement>,
}
