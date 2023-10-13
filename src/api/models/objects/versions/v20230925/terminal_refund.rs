//! TerminalRefund

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::action_cancel_reason::ActionCancelReasonV20230925;

use super::money::MoneyV20230925;

/// Represents a payment refund processed by the Square Terminal.
///
/// Only supports Interac (Canadian debit network) payment refunds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalRefundV20230925 {
    /// A unique ID for this `TerminalRefund`.
    ///
    /// Min Length 10 Max Length 255
    pub id: Option<String>,
    /// The reference to the payment refund created by completing this `TerminalRefund`.
    pub refund_id: Option<String>,
    /// The unique ID of the payment being refunded.
    ///
    /// Min Length 1
    pub payment_id: String,
    /// Read only The reference to the Square order ID for the payment identified by the `payment_id`.
    pub order_id: Option<String>,
    /// The amount of money, inclusive of `tax_money`, that the `TerminalRefund` should return. This value is limited to the amount taken in the original payment minus any completed or pending refunds.
    pub amount_money: MoneyV20230925,
    /// A description of the reason for the refund.
    ///
    /// Max Length 192
    pub reason: String,
    /// The unique ID of the device intended for this `TerminalRefund`. The Id can be retrieved from `/v2/devices` api.
    pub device_id: String,
    /// The RFC 3339 duration, after which the refund is automatically canceled. A `TerminalRefund` that is `PENDING` is automatically `CANCELED` and has a cancellation reason of `TIMED_OUT`.
    ///
    /// Default: 5 minutes from creation.
    ///
    /// Maximum: 5 minutes
    ///
    /// Example for 2 days, 12 hours, 30 minutes, and 15 seconds: `P2DT12H30M15S`
    pub deadline_duration: Option<String>,
    /// Read only The status of the `TerminalRefund`. Options: `PENDING`, `IN_PROGRESS`, `CANCEL_REQUESTED`, `CANCELED`, or `COMPLETED`.
    pub status: Option<String>,
    /// Read only Present if the status is `CANCELED`.
    pub cancel_reason: Option<ActionCancelReasonV20230925>,
    /// Read only The time when the `TerminalRefund` was created, as an RFC 3339 timestamp.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// Read only The time when the `TerminalRefund` was last updated, as an RFC 3339 timestamp.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
    /// Read only The ID of the application that created the refund.
    pub app_id: Option<String>,
    /// Read only The location of the device where the `TerminalRefund` was directed.
    ///
    /// Max Length 64
    pub location_id: Option<String>,
}
