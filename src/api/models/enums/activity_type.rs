//! ActivityType Enum

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityType {
    /// A manual adjustment applied to the seller's account by Square.
    Adjustment,
    /// A refund for an application fee on a payment.
    AppFeeRefund,
    /// Revenue generated from an application fee on a payment.
    AppFeeRevenue,
    /// An automatic transfer from the payment processing balance to the Square Savings account. These are, generally, proportional to the seller's sales.
    AutomaticSavings,
    /// An automatic transfer from the Square Savings account back to the processing balance. These are, generally, proportional to the seller's refunds.
    AutomaticSavingsReversed,
    /// A credit card payment capture.
    Charge,
    /// Any fees involved with deposits such as instant deposits.
    DepositFee,
    /// The balance change due to a dispute event.
    Dispute,
    /// An escheatment entry for remittance.
    Escheatment,
    /// The Square processing fee.
    Fee,
    /// Square offers free payments processing for a variety of business scenarios, including seller referrals or when Square wants to apologize (for example, for a bug, customer service, or repricing complication). This entry represents a credit to the seller for the purposes of free processing.
    FreeProcessing,
    /// An adjustment made by Square related to holding a payment.
    HoldAdjustment,
    /// An external change to a seller's balance. Initial, in the sense that it causes the creation of the other activity types, such as hold and refund.
    InitialBalanceChange,
    /// The balance change from a money transfer.
    MoneyTransfer,
    /// The reversal of a money transfer.
    MoneyTransferReversal,
    /// The balance change for a chargeback that has been filed.
    OpenDispute,
    /// Any other type that does not belong in the rest of the types.
    Other,
    /// Any other type of adjustment that does not fall under existing types.
    OtherAdjustment,
    /// A fee paid to a third-party merchant.
    PaidServiceFee,
    /// A fee paid to a third-party merchant.
    PaidServiceFeeRefund,
    /// Repayment for a redemption code.
    RedemptionCode,
    /// A refund for an existing card payment.
    Refund,
    /// An adjustment made by Square related to releasing a payment.
    ReleaseAdjustment,
    /// Fees paid for funding risk reserve.
    ReserveHold,
    /// Fees released from risk reserve.
    ReserveRelease,
    /// An entry created when Square receives a response for the ACH file that Square sent indicating that the settlement of the original entry failed.
    ReturnedPayout,
    /// A capital merchant cash advance (MCA) assessment. These are, generally, proportional to the merchant's sales but can be issued for other reasons related to the MCA.
    SquareCapitalPayment,
    /// A capital merchant cash advance (MCA) assessment refund. These are, generally, proportional to the merchant's refunds but can be issued for other reasons related to the MCA.
    SquareCapitalReversedPayment,
    /// A fee charged for subscription to a Square product.
    SubscriptionFee,
    /// A Square subscription fee that has been refunded.
    SubscriptionFeePaidRefund,
    /// The refund of a previously charged Square product subscription fee.
    SubscriptionFeeRefund,
    /// The tax paid on fee amounts.
    TaxOnFee,
    /// Fees collected by a third-party platform.
    ThirdPartyFee,
    /// Refunded fees from a third-party platform.
    ThirdPartyFeeRefund,
    /// Balance change due to money transfer.
    Payout,
    /// Indicates the withholding of a portion of each payment by Square that has been automatically converted into bitcoin using Cash App. The seller manages their bitcoin in their Cash App account.
    AutomaticBitcoinConversions,
    /// Indicates a return of the payment withholding that had been scheduled to be converted into bitcoin using Cash App to the Square payments balance.
    AutomaticBitcoinConversionsReversed,
    /// The repayment made toward the outstanding balance on the seller's Square credit card.
    CreditCardRepayment,
    /// The reversal of the repayment made toward the outstanding balance on the seller's Square credit card.
    CreditCardRepaymentReversed,
}
