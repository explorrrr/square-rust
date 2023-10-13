//! Coordinates

use serde::{Deserialize, Serialize};

/// Latitude and longitude coordinates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    /// The latitude of the coordinate expressed in degrees.
    pub latitude: Option<f64>,
    /// The longitude of the coordinate expressed in degrees.
    pub longitude: Option<f64>,
}
