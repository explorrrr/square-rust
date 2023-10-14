//! GiftCardActivity

use serde::{Deserialize, Serialize};

use super::{
    gift_card_activity_activate::GiftCardActivityActivateV20230925,
    gift_card_activity_adjust_decrement::GiftCardActivityAdjustDecrementV20230925,
    gift_card_activity_adjust_increment::GiftCardActivityAdjustIncrementV20230925,
    gift_card_activity_block::GiftCardActivityBlockV20230925,
    gift_card_activity_clear_balance::GiftCardActivityClearBalanceV20230925,
    gift_card_activity_deactivate::GiftCardActivityDeactivateV20230925,
    gift_card_activity_import::GiftCardActivityImportV20230925,
    gift_card_activity_import_reversal::GiftCardActivityImportReversalV20230925,
    gift_card_activity_load::GiftCardActivityLoadV20230925, gift_card_activity_redeem::GiftCardActivityRedeemV20230925,
    gift_card_activity_refund::GiftCardActivityRefundV20230925,
    gift_card_activity_transfer_balance_from::GiftCardActivityTransferBalanceFromV20230925,
    gift_card_activity_transfer_balance_to::GiftCardActivityTransferBalanceToV20230925,
    gift_card_activity_unblock::GiftCardActivityUnblockV20230925,
    gift_card_activity_unlinked_activity_refund::GiftCardActivityUnlinkedActivityRefundV20230925,
    money::MoneyV20230925,
};
use crate::api::models::enums::versions::v20230925::gift_card_activity_type::GiftCardActivityTypeV20230925;

/// Represents an action performed on a [gift card](https://developer.squareup.com/reference/square/objects/GiftCard) that affects its state or balance.
///
/// A gift card activity contains information about a specific activity type. For example, a REDEEM activity includes a `redeem_activity_details` field that contains information about the redemption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityV20230925 {
    /// Read only The Square-assigned ID of the gift card activity.
    pub id: Option<String>,
    /// The type of gift card activity.
    pub r#type: GiftCardActivityTypeV20230925,
    /// The ID of the [business location](https://developer.squareup.com/reference/square/objects/Location) where the activity occurred.
    pub location_id: String,
    /// Read only The timestamp when the gift card activity was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// The gift card ID. When creating a gift card activity, gift_card_id is not required if gift_card_gan is specified.
    pub gift_card_id: Option<String>,
    /// The gift card account number (GAN). When creating a gift card activity, gift_card_gan is not required if gift_card_id is specified.
    pub gift_card_gan: Option<String>,
    /// Read only The final balance on the gift card after the action is completed.
    pub gift_card_balance_money: Option<MoneyV20230925>,
    /// Additional details about a LOAD activity, which is used to reload money onto a gift card.
    pub load_activity_details: Option<GiftCardActivityLoadV20230925>,
    /// Additional details about an ACTIVATE activity, which is used to activate a gift card with an initial balance.
    pub activate_activity_details: Option<GiftCardActivityActivateV20230925>,
    /// Additional details about a REDEEM activity, which is used to redeem a gift card for a purchase.
    ///
    /// For applications that process payments using the Square Payments API, Square creates a REDEEM activity that updates the gift card balance after the corresponding [CreatePayment](https://developer.squareup.com/reference/square/payments-api/create-payment) request is completed. Applications that use a custom payment processing system must call [CreateGiftCardActivity](https://developer.squareup.com/reference/square/giftcardactivities-api/create-gift-card-activity) to create the REDEEM activity.
    pub redeem_activity_details: Option<GiftCardActivityRedeemV20230925>,
    /// Additional details about a CLEAR_BALANCE activity, which is used to set the balance of a gift card to zero.
    pub clear_balance_activity_details: Option<GiftCardActivityClearBalanceV20230925>,
    /// Additional details about a DEACTIVATE activity, which is used to deactivate a gift card.
    pub deactivate_activity_details: Option<GiftCardActivityDeactivateV20230925>,
    /// Additional details about an ADJUST_INCREMENT activity, which is used to add money to a gift card outside of a typical ACTIVATE, LOAD, or REFUND activity flow.
    pub adjust_increment_activity_details: Option<GiftCardActivityAdjustIncrementV20230925>,
    /// Additional details about an ADJUST_DECREMENT activity, which is used to deduct money from a gift card outside of a typical REDEEM activity flow.
    pub adjust_decrement_activity_details: Option<GiftCardActivityAdjustDecrementV20230925>,
    /// Additional details about a REFUND activity, which is used to add money to a gift card when refunding a payment.
    ///
    /// For applications that process payments using the Square Payments API, Square creates a REFUND activity that updates the gift card balance after the corresponding [RefundPayment](https://developer.squareup.com/reference/square/refunds-api/refund-payment) request is completed. Applications that use a custom payment processing system must call [CreateGiftCardActivity](https://developer.squareup.com/reference/square/giftcardactivities-api/create-gift-card-activity) to create the REFUND activity.
    pub refund_activity_details: Option<GiftCardActivityRefundV20230925>,
    /// Additional details about an UNLINKED_ACTIVITY_REFUND activity. This activity is used to add money to a gift card when refunding a payment that was processed using a custom payment processing system and not linked to the gift card.
    pub unlinked_activity_refund_activity_details: Option<GiftCardActivityUnlinkedActivityRefundV20230925>,
    /// Read only Additional details about an IMPORT activity, which Square uses to import a third-party gift card with a balance.
    pub import_activity_details: Option<GiftCardActivityImportV20230925>,
    /// Read only Additional details about a BLOCK activity, which Square uses to temporarily block a gift card.
    pub block_activity_details: Option<GiftCardActivityBlockV20230925>,
    /// Read only Additional details about an UNBLOCK activity, which Square uses to unblock a gift card.
    pub unblock_activity_details: Option<GiftCardActivityUnblockV20230925>,
    /// Read only Additional details about an IMPORT_REVERSAL activity, which Square uses to reverse the import of a third-party gift card.
    pub import_reversal_activity_details: Option<GiftCardActivityImportReversalV20230925>,
    /// Read only Additional details about a TRANSFER_BALANCE_TO activity, which Square uses to add money to a gift card as the result of a transfer from another gift card.
    pub transfer_balance_to_activity_details: Option<GiftCardActivityTransferBalanceToV20230925>,
    /// Read only Additional details about a TRANSFER_BALANCE_FROM activity, which Square uses to deduct money from a gift as the result of a transfer to another gift card.
    pub transfer_balance_from_activity_details: Option<GiftCardActivityTransferBalanceFromV20230925>,
}
