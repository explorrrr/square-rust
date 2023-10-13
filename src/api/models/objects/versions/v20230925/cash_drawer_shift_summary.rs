//! CashDrawerShiftSummary

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::cash_drawer_shift_state::CashDrawerShiftStateV20230925;
use crate::api::models::objects::versions::v20230925::money::MoneyV20230925;

/// The summary of a closed cash drawer shift.
///
/// This model contains only the money counted to start a cash drawer shift, counted at the end of the shift, and the amount that should be in the drawer at shift end based on summing all cash drawer shift events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashDrawerShiftSummaryV20230925 {
    /// The shift unique ID.
    pub id: Option<String>,
    /// The shift current state.
    pub state: Option<CashDrawerShiftStateV20230925>,
    /// The shift start time in ISO 8601 format.
    pub opened_at: Option<String>,
    /// The shift end time in ISO 8601 format.
    pub ended_at: Option<String>,
    /// The shift close time in ISO 8601 format.
    pub closed_at: Option<String>,
    /// An employee free-text description of a cash drawer shift.
    pub description: Option<String>,
    /// The amount of money in the cash drawer at the start of the shift. This must be a positive amount.
    pub opened_cash_money: Option<MoneyV20230925>,
    /// The amount of money that should be in the cash drawer at the end of the shift, based on the cash drawer events on the shift. The amount is correct if all shift employees accurately recorded their cash drawer shift events. Unrecorded events and events with the wrong amount result in an incorrect expected_cash_money amount that can be negative.
    pub expected_cash_money: Option<MoneyV20230925>,
    /// The amount of money found in the cash drawer at the end of the shift by an auditing employee. The amount must be greater than or equal to zero.
    pub closed_cash_money: Option<MoneyV20230925>,
    /// Read only The shift start time in RFC 3339 format.
    pub created_at: Option<String>,
    /// Read only The shift updated at time in RFC 3339 format.
    pub updated_at: Option<String>,
    /// Read only The ID of the location the cash drawer shift belongs to.
    pub location_id: Option<String>,
}
