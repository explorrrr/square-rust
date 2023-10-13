//! CashDrawerEventType enum

use serde::{Deserialize, Serialize};

/// The types of events on a CashDrawerShift.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CashDrawerEventTypeV20230925 {
    /// Triggered when a no sale occurs on a cash drawer. A CashDrawerEvent of this type must have a zero money amount.
    NoSale,
    /// Triggered when a cash tender payment occurs on a cash drawer. A CashDrawerEvent of this type can must not have a negative amount.
    CashTenderPayment,
    /// Triggered when a check, gift card, or other non-cash payment occurs on a cash drawer. A CashDrawerEvent of this type must have a zero money amount.
    OtherTenderPayment,
    /// Triggered when a split tender bill is cancelled after cash has been tendered. A CASH_TENDER_CANCELLED_PAYMENT should have a corresponding CASH_TENDER_PAYMENT. A CashDrawerEvent of this type must not have a negative amount.
    CashTenderCancelledPayment,
    /// Triggered when a split tender bill is cancelled after a non-cash tender has been tendered. An OTHER_TENDER_CANCELLED_PAYMENT should have a corresponding OTHER_TENDER_PAYMENT. A CashDrawerEvent of this type must have a zero money amount.
    OtherTenderCancelledPayment,
    /// Triggered when a cash tender refund occurs. A CashDrawerEvent of this type must not have a negative amount.
    CashTenderRefund,
    /// Triggered when an other tender refund occurs. A CashDrawerEvent of this type must have a zero money amount.
    OtherTenderRefund,
    /// Triggered when money unrelated to a payment is added to the cash drawer. For example, an employee adds coins to the drawer. A CashDrawerEvent of this type must not have a negative amount.
    PaidIn,
    /// Triggered when money is removed from the drawer for other reasons than making change. For example, an employee pays a delivery person with cash from the cash drawer. A CashDrawerEvent of this type must not have a negative amount.
    PaidOut,
}
