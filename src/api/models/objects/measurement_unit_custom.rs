//! MeasurementUnitCustom

use serde::{Deserialize, Serialize};

/// The information needed to define a custom unit, provided by the seller.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementUnitCustom {
    /// The name of the custom unit, for example "bushel".
    pub name: String,
    /// The abbreviation of the custom unit, such as "bsh" (bushel). This appears in the cart for the Point of Sale app, and in reports.
    pub abbreviation: String,
}
