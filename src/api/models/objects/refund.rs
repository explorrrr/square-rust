//! Refund

use serde::{Deserialize, Serialize};

/// Represents a refund processed for a Square transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refund {
    /// The refund's unique ID.
    ///
    /// Max Length: 255
    pub id: String,
    /// The ID of the refund's associated location.
    ///
    /// Max Length: 50
    pub location_id: String,
    /// The ID of the transaction that the refunded tender is part of.
    ///
    /// Max Length: 192
    pub transaction_id: Option<String>,
    /// The ID of the refunded tender.
    ///
    /// Max Length: 192
    pub tender_id: String,
    /// The timestamp for when the refund was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// * UTC: 2020-01-26T02:25:34Z
    ///
    /// * Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// The reason for the refund being issued.
    ///
    /// Max Length: 192
    pub reason: String,
    /// The amount of money refunded to the buyer.
    pub amount_money: Money,
    /// The current status of the refund (PENDING, APPROVED, REJECTED, or FAILED).
    pub status: RefundStatus,
    /// The amount of Square processing fee money refunded to the merchant.
    pub processing_fee_money: Option<Money>,
}
