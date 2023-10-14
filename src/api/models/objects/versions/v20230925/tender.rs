//! Tender

use serde::{Deserialize, Serialize};

use super::{
    money::MoneyV20230925, tender_bank_account_details::TenderBankAccountDetailsV20230925,
    tender_buy_now_pay_later_details::TenderBuyNowPayLaterDetailsV20230925,
    tender_card_details::TenderCardDetailsV20230925, tender_cash_details::TenderCashDetailsV20230925,
    tender_square_account_details::TenderSquareAccountDetailsV20230925,
};
use crate::api::models::enums::versions::v20230925::tender_type::TenderTypeV20230925;

/// Represents a tender (i.e., a method of payment) used in a Square transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenderV20230925 {
    /// The tender's unique ID. It is the associated payment ID.
    ///
    /// Max Length 192
    pub id: Option<String>,
    /// The ID of the transaction's associated location.
    ///
    /// Max Length 50
    pub location_id: Option<String>,
    /// The ID of the tender's associated transaction.
    ///
    /// Max Length 192
    pub transaction_id: Option<String>,
    /// Read only The timestamp for when the tender was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    ///
    /// Max Length 32
    pub created_at: Option<String>,
    /// An optional note associated with the tender at the time of payment.
    ///
    /// Max Length 500
    pub note: Option<String>,
    /// The total amount of the tender, including tip_money. If the tender has a payment_id, the total_money of the corresponding [Payment](https://developer.squareup.com/reference/square/objects/Payment) will be equal to the amount_money of the tender.
    pub amount_money: Option<MoneyV20230925>,
    /// The tip's amount of the tender.
    pub tip_money: Option<MoneyV20230925>,
    /// The amount of any Square processing fees applied to the tender.
    ///
    /// This field is not immediately populated when a new transaction is created. It is usually available after about ten seconds.
    pub processing_fee_money: Option<MoneyV20230925>,
    /// If the tender is associated with a customer or represents a customer's card on file, this is the ID of the associated customer.
    ///
    /// Max Length 191
    pub customer_id: Option<String>,
    /// The type of tender, such as `CARD` or `CASH`.
    pub r#type: TenderTypeV20230925,
    /// The details of the card tender.
    ///
    /// This value is present only if the value of `type` is `CARD`.
    pub card_details: Option<TenderCardDetailsV20230925>,
    /// The details of the cash tender.
    ///
    /// The value of this field is present only if the value of `type` is `CASH`.
    pub cash_details: Option<TenderCashDetailsV20230925>,
    /// The details of the bank account tender.
    ///
    /// This value is present only if the value of `type` is `BANK_ACCOUNT`.
    pub bank_account_details: Option<TenderBankAccountDetailsV20230925>,
    /// The details of a Buy Now Pay Later tender.
    ///
    /// This value is present only if the value of `type` is `BUY_NOW_PAY_LATER`.
    pub buy_now_pay_later_details: Option<TenderBuyNowPayLaterDetailsV20230925>,
    /// The details of a Square Account tender.
    ///
    /// This value is present only if the value of `type` is `SQUARE_ACCOUNT`.
    pub square_account_details: Option<TenderSquareAccountDetailsV20230925>,
    /// The ID of the [Payment](https://developer.squareup.com/reference/square/objects/Payment) that corresponds to this tender. This value is only present for payments created with the v2 Payments API.
    ///
    /// Max Length 192
    pub payment_id: Option<String>,
}
