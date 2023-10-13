//! OrderLineItemAppliedServiceCharge

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLineItemAppliedServiceChargeV20230925 {
    /// A unique ID that identifies the applied service charge only within this order.
    ///
    /// Max Length 60
    pub uid: Option<String>,
    /// The uid of the service charge that the applied service charge represents. It must reference a service charge present in the order.service_charges field.
    ///
    /// This field is immutable. To change which service charges apply to a line item, delete and add a new OrderLineItemAppliedServiceCharge.
    ///
    /// Min Length 1 Max Length 60
    pub service_charge_uid: String,
    /// Read only The amount of money applied by the service charge to the line item.
    pub applied_money: Option<MoneyV20230925>,
}
