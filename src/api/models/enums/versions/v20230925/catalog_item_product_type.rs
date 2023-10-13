//! CatalogItemProductType Enum

use serde::{Deserialize, Serialize};

/// The type of a CatalogItem.
/// Connect V2 only allows the creation of REGULAR or APPOINTMENTS_SERVICE items.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogItemProductTypeV20230925 {
    /// An ordinary item.
    Regular,
    /// A service that can be booked using the Square Appointments app.
    AppointmentsService,
}
