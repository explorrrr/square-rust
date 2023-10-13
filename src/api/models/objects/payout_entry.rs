//! PayoutEntry

use serde::{Deserialize, Serialize};

/// One or more `PayoutEntry` objects that make up a `Payout`.
///
/// Each one has a date, amount, and type of activity. The total amount of the payout will equal the sum of the payout entries for a batch payout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutEntry {
    /// A unique ID for the payout entry.
    ///
    /// Min Length 1
    pub id: String,
    /// The ID of the payout entries' associated payout.
    ///
    /// Min Length 1
    pub payout_id: String,
    /// The timestamp of when the payout entry affected the balance, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// - UTC: 2020-01-26T02:25:34Z
    /// - Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub effective_at: Option<String>,
    /// The type of activity associated with this payout entry.
    pub r#type: Option<ActivityType>,
    /// The amount of money involved in this payout entry.
    pub gross_amount_money: Option<Money>,
    /// The amount of Square fees associated with this payout entry.
    pub fee_amount_money: Option<Money>,
    /// The net proceeds from this transaction after any fees.
    pub net_amount_money: Option<Money>,
    /// Details of any developer app fee revenue generated on a payment.
    pub type_app_fee_revenue_details: Option<PaymentBalanceActivityAppFeeRevenueDetail>,
    /// Details of a refund for an app fee on a payment.
    pub type_app_fee_refund_details: Option<PaymentBalanceActivityAppFeeRefundDetail>,
    /// Details of any automatic transfer from the payment processing balance to the Square Savings account.
    pub type_automatic_savings_details: Option<PaymentBalanceActivityAutomaticSavingsDetail>,
    /// Details of any automatic transfer from the Square Savings account back to the processing balance.
    pub type_automatic_savings_reversed_details: Option<PaymentBalanceActivityAutomaticSavingsReversedDetail>,
    /// Details of credit card payment captures.
    pub type_charge_details: Option<PaymentBalanceActivityChargeDetail>,
    /// Details of any fees involved with deposits such as for instant deposits.
    pub type_deposit_fee_details: Option<PaymentBalanceActivityDepositFeeDetail>,
    /// Details of any balance change due to a dispute event.
    pub type_dispute_details: Option<PaymentBalanceActivityDisputeDetail>,
    /// Details of adjustments due to the Square processing fee.
    pub type_fee_details: Option<PaymentBalanceActivityFeeDetail>,
    /// Square offers Free Payments Processing for a variety of business scenarios including seller referral or when Square wants to apologize for a bug, customer service, repricing complication, and so on. This entry represents details of any credit to the merchant for the purposes of Free Processing.
    pub type_free_processing_details: Option<PaymentBalanceActivityFreeProcessingDetail>,
    /// Details of any adjustment made by Square related to the holding or releasing of a payment.
    pub type_hold_adjustment_details: Option<PaymentBalanceActivityHoldAdjustmentDetail>,
    /// Details of any open disputes.
    pub type_open_dispute_details: Option<PaymentBalanceActivityOpenDisputeDetail>,
    /// Details of any other type that does not belong in the rest of the types.
    pub type_other_details: Option<PaymentBalanceActivityOtherDetail>,
    /// Details of any other type of adjustments that don't fall under existing types.
    pub type_other_adjustment_details: Option<PaymentBalanceActivityOtherAdjustmentDetail>,
    /// Details of a refund for an existing card payment.
    pub type_refund_details: Option<PaymentBalanceActivityRefundDetail>,
    /// Details of fees released for adjustments.
    pub type_release_adjustment_details: Option<PaymentBalanceActivityReleaseAdjustmentDetail>,
    /// Details of fees paid for funding risk reserve.
    pub type_reserve_hold_details: Option<PaymentBalanceActivityReserveHoldDetail>,
    /// Details of fees released from risk reserve.
    pub type_reserve_release_details: Option<PaymentBalanceActivityReserveReleaseDetail>,
    /// Details of capital merchant cash advance (MCA) assessments. These are, generally, proportional to the merchant's sales but may be issued for other reasons related to the MCA.
    pub type_square_capital_payment_details: Option<PaymentBalanceActivitySquareCapitalPaymentDetail>,
    /// Details of capital merchant cash advance (MCA) assessment refunds. These are, generally, proportional to the merchant's refunds but may be issued for other reasons related to the MCA.
    pub type_square_capital_reversed_payment_details: Option<PaymentBalanceActivitySquareCapitalReversedPaymentDetail>,
    /// Details of tax paid on fee amounts.
    pub type_tax_on_fee_details: Option<PaymentBalanceActivityTaxOnFeeDetail>,
    /// Details of fees collected by a 3rd party platform.
    pub type_third_party_fee_details: Option<PaymentBalanceActivityThirdPartyFeeDetail>,
    /// Details of refunded fees from a 3rd party platform.
    pub type_third_party_fee_refund_details: Option<PaymentBalanceActivityThirdPartyFeeRefundDetail>,
}
