//! CatalogTax

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::tax_calculation_phase::TaxCalculationPhaseV20230925;
use crate::api::models::enums::versions::v20230925::tax_inclusion_type::TaxInclusionTypeV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogTaxV20230925 {
    /// The tax's name. This is a searchable attribute for use in applicable query filters, and its value length is of Unicode code points.
    ///
    /// Max Length 255
    pub name: Option<String>,
    /// Whether the tax is calculated based on a payment's subtotal or total.
    pub calculation_phase: Option<TaxCalculationPhaseV20230925>,
    /// Whether the tax is ADDITIVE or INCLUSIVE.
    pub inclusion_type: Option<TaxInclusionTypeV20230925>,
    /// The percentage of the tax in decimal form, using a '.' as the decimal separator and without a '%' sign. A value of 7.5 corresponds to 7.5%. For a location-specific tax rate, contact the tax authority of the location or a tax consultant.
    pub percentage: Option<String>,
    /// If true, the fee applies to custom amounts entered into the Square Point of Sale app that are not associated with a particular CatalogItem.
    pub applies_to_custom_amounts: Option<bool>,
    /// A Boolean flag to indicate whether the tax is displayed as enabled (true) in the Square Point of Sale app or not (false).
    pub enabled: Option<bool>,
    /// The ID of a CatalogProductSet object. If set, the tax is applicable to all products in the product set.
    pub applies_to_product_set_id: Option<String>,
}
