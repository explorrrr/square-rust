//! OrderReturnServiceCharge

use serde::{Deserialize, Serialize};

use super::{money::MoneyV20230925, order_line_item_applied_tax::OrderLineItemAppliedTaxV20230925};
use crate::api::models::enums::versions::v20230925::{
    order_service_charge_calculation_phase::OrderServiceChargeCalculationPhaseV20230925,
    order_service_charge_scope::OrderServiceChargeScopeV20230925,
    order_service_charge_treatment_type::OrderServiceChargeTreatmentTypeV20230925,
};

/// Represents the service charge applied to the original order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderReturnServiceChargeV20230925 {
    /// A unique ID that identifies the return service charge only within this order.
    ///
    /// Max Length 60
    pub uid: Option<String>,
    /// The service charge uid from the order containing the original service charge. source_service_charge_uid is null for unlinked returns.
    ///
    /// Max Length 60
    pub source_service_charge_uid: Option<String>,
    /// The name of the service charge.
    ///
    /// Max Length 255
    pub name: Option<String>,
    /// The catalog object ID of the associated [OrderServiceCharge](https://developer.squareup.com/reference/square/objects/OrderServiceCharge).
    ///
    /// Max Length 192
    pub catalog_object_id: Option<String>,
    /// The version of the catalog object that this service charge references.
    pub catalog_version: Option<i64>,
    /// The percentage of the service charge, as a string representation of a decimal number. For example, a value of "7.25" corresponds to a percentage of 7.25%.
    ///
    /// Either percentage or amount_money should be set, but not both.
    ///
    /// Max Length 10
    pub percentage: Option<String>,
    /// The amount of a non-percentage-based service charge.
    ///
    /// Either percentage or amount_money should be set, but not both.
    pub amount_money: Option<MoneyV20230925>,
    /// Read only The amount of money applied to the order by the service charge, including any inclusive tax amounts, as calculated by Square.
    ///
    /// For fixed-amount service charges, applied_money is equal to amount_money.
    /// For percentage-based service charges, applied_money is the money calculated using the percentage.
    pub applied_money: Option<MoneyV20230925>,
    /// Read only The total amount of money to collect for the service charge.
    ///
    /// NOTE: If an inclusive tax is applied to the service charge, total_money does not equal applied_money plus total_tax_money because the inclusive tax amount is already included in both applied_money and total_tax_money.
    pub total_money: Option<MoneyV20230925>,
    /// Read only The total amount of tax money to collect for the service charge.
    pub total_tax_money: Option<MoneyV20230925>,
    /// Read only The calculation phase after which to apply the service charge.
    pub calculation_phase: Option<OrderServiceChargeCalculationPhaseV20230925>,
    /// Indicates whether the surcharge can be taxed. Service charges calculated in the TOTAL_PHASE cannot be marked as taxable.
    pub taxable: Option<bool>,
    /// The list of references to OrderReturnTax entities applied to the OrderReturnServiceCharge. Each OrderLineItemAppliedTax has a `tax_uid` that references the `uid` of a top-level OrderReturnTax that is being applied to the OrderReturnServiceCharge. On reads, the applied amount is populated.
    pub applied_taxes: Option<Vec<OrderLineItemAppliedTaxV20230925>>,
    /// Read only The treatment type of the service charge.
    pub treatment_type: Option<OrderServiceChargeTreatmentTypeV20230925>,
    /// Indicates the level at which the apportioned service charge applies. For ORDER scoped service charges, Square generates references in `applied_service_charges` on all order line items that do not have them. For LINE_ITEM scoped service charges, the service charge only applies to line items with a service charge reference in their `applied_service_charges` field.
    ///
    /// This field is immutable. To change the scope of an apportioned service charge, you must delete the apportioned service charge and re-add it as a new apportioned service charge.
    pub scope: Option<OrderServiceChargeScopeV20230925>,
}
