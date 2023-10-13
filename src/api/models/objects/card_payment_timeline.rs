//! CardPaymentTimeline

use serde::{Deserialize, Serialize};

/// The timeline for card payments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardPaymentTimeline {
    /// The timestamp when the payment was authorized, in RFC 3339 format.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub authorized_at: Option<String>,
    /// The timestamp when the payment was captured, in RFC 3339 format.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub captured_at: Option<String>,
    /// The timestamp when the payment was voided, in RFC 3339 format.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub voided_at: Option<String>,
}
