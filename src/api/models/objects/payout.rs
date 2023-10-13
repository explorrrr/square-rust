//! Payout

use serde::{Deserialize, Serialize};

/// An accounting of the amount owed the seller and record of the actual transfer to their external bank account or to the Square balance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payout {
    /// A unique ID for the payout.
    ///
    /// Min Length 1
    pub id: String,
    /// Indicates the payout status.
    pub status: Option<PayoutStatus>,
    /// The ID of the location associated with the payout.
    ///
    /// Min Length 1
    pub location_id: String,
    /// The timestamp of when the payout was created and submitted for deposit to the seller's banking destination, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// The timestamp of when the payout was last updated, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
    /// The amount of money involved in the payout. A positive amount indicates a deposit, and a negative amount indicates a withdrawal. This amount is never zero.
    pub amount_money: Option<Money>,
    /// Information about the banking destination (such as a bank account, Square checking account, or debit card) against which the payout was made.
    pub destination: Option<Destination>,
    /// The version number, which is incremented each time an update is made to this payout record. The version number helps developers receive event notifications or feeds out of order.
    pub version: Option<i32>,
    /// Indicates the payout type.
    pub r#type: Option<PayoutType>,
    /// A list of transfer fees and any taxes on the fees assessed by Square for this payout.
    pub payout_fee: Option<Vec<PayoutFee>>,
    /// The calendar date, in ISO 8601 format (YYYY-MM-DD), when the payout is due to arrive in the seller’s banking destination.
    pub arrival_date: Option<String>,
    /// A unique ID for each Payout object that might also appear on the seller’s bank statement. You can use this ID to automate the process of reconciling each payout with the corresponding line item on the bank statement.
    pub end_to_end_id: Option<String>,
}
