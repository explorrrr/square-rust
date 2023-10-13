//! Product Enum

use serde::{Deserialize, Serialize};

/// Indicates the Square product used to generate a change.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Product {
    /// Square Point of Sale application.
    SquarePos,
    /// Square Connect APIs (for example, Orders API or Checkout API).
    ExternalApi,
    /// A Square subscription (various products).
    Billing,
    /// Square Appointments.
    Appointments,
    /// Square Invoices.
    Invoices,
    /// Square Online Store.
    OnlineStore,
    /// Square Payroll.
    Payroll,
    /// Square Dashboard.
    Dashboard,
    /// Item Library Import.
    ItemLibraryImport,
    /// A Square product that does not match any other value.
    Other,
}
