//! CashDrawerShift

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::cash_drawer_shift_state::CashDrawerShiftStateV20230925;
use crate::api::models::objects::versions::v20230925::cash_drawer_device::CashDrawerDeviceV20230925;
use crate::api::models::objects::versions::v20230925::money::MoneyV20230925;

/// This model gives the details of a cash drawer shift.
///
/// The cash_payment_money, cash_refund_money, cash_paid_in_money, and cash_paid_out_money fields are all computed by summing their respective event types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashDrawerShiftV20230925 {
    /// The shift unique ID.
    pub id: Option<String>,
    /// The shift current state.
    pub state: Option<CashDrawerShiftStateV20230925>,
    /// The time when the shift began, in ISO 8601 format.
    pub opened_at: Option<String>,
    /// The time when the shift ended, in ISO 8601 format.
    pub ended_at: Option<String>,
    /// The time when the shift was closed, in ISO 8601 format.
    pub closed_at: Option<String>,
    /// The free-form text description of a cash drawer by an employee.
    pub description: Option<String>,
    /// The amount of money in the cash drawer at the start of the shift. The amount must be greater than or equal to zero.
    pub opened_cash_money: Option<MoneyV20230925>,
    /// The amount of money added to the cash drawer from cash payments. This is computed by summing all events with the types CASH_TENDER_PAYMENT and CASH_TENDER_CANCELED_PAYMENT. The amount is always greater than or equal to zero.
    pub cash_payment_money: Option<MoneyV20230925>,
    /// The amount of money removed from the cash drawer from cash refunds. It is computed by summing the events of type CASH_TENDER_REFUND. The amount is always greater than or equal to zero.
    pub cash_refunds_money: Option<MoneyV20230925>,
    /// The amount of money added to the cash drawer for reasons other than cash payments. It is computed by summing the events of type PAID_IN. The amount is always greater than or equal to zero.
    pub cash_paid_in_money: Option<MoneyV20230925>,
    /// The amount of money removed from the cash drawer for reasons other than cash refunds. It is computed by summing the events of type PAID_OUT. The amount is always greater than or equal to zero.
    pub cash_paid_out_money: Option<MoneyV20230925>,
    /// The amount of money that should be in the cash drawer at the end of the shift, based on the shift's other money amounts. This can be negative if employees have not correctly recorded all the events on the cash drawer. cash_paid_out_money is a summation of amounts from cash_payment_money (zero or positive), cash_refunds_money (zero or negative), cash_paid_in_money (zero or positive), and cash_paid_out_money (zero or negative) event types.
    pub expected_cash_money: Option<MoneyV20230925>,
    /// The amount of money found in the cash drawer at the end of the shift by an auditing employee. The amount should be positive.
    pub closed_cash_money: Option<MoneyV20230925>,
    /// The device running Square Point of Sale that was connected to the cash drawer.
    pub device: Option<CashDrawerDeviceV20230925>,
    /// Read only The shift start time in RFC 3339 format.
    pub created_at: Option<String>,
    /// Read only The shift updated at time in RFC 3339 format.
    pub updated_at: Option<String>,
    /// Read only The ID of the location the cash drawer shift belongs to.
    pub location_id: Option<String>,
    /// Read only The IDs of all team members that were logged into Square Point of Sale at any point while the cash drawer shift was open.
    pub team_member_ids: Option<Vec<String>>,
    /// Read only The ID of the team member that started the cash drawer shift.
    pub opening_team_member_id: Option<String>,
    /// Read only The ID of the team member that ended the cash drawer shift.
    pub ending_team_member_id: Option<String>,
    /// Read only The ID of the team member that closed the cash drawer shift by auditing the cash drawer contents.
    pub closing_team_member_id: Option<String>,
}
