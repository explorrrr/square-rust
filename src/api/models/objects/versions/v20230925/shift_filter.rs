//! ShiftFilter

use serde::{Deserialize, Serialize};



use crate::api::models::enums::versions::v20230925::shift_filter_status::ShiftFilterStatusV20230925;

use super::{shift_wage::ShiftWageV20230925, square_break::SquareBreakV20230925, time_range::TimeRangeV20230925, shift_workday::ShiftWorkdayV20230925};


/// Defines a filter used in a search for `Shift` records.
///
/// AND logic is used by Square's servers to apply each filter property specified.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftFilterV20230925 {
    /// Fetch shifts for the specified location.
    pub location_ids: Option<Vec<String>>,
    /// Fetch a Shift instance by `Shift.status`.
    pub status: Option<Vec<ShiftFilterStatusV20230925>>,
    /// Fetch Shift instances that start in the time range - Inclusive.
    pub start: Option<TimeRangeV20230925>,
    /// Fetch the Shift instances that end in the time range - Inclusive.
    pub end: Option<TimeRangeV20230925>,
    /// Fetch the Shift instances based on the workday date range.
    pub workday: Option<ShiftWorkdayV20230925>,
    /// Fetch shifts for the specified team members. Replaced `employee_ids` at version "2020-08-26".
    pub team_member_ids: Option<Vec<String>>,
}
