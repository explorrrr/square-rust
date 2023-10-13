//! TerminalAction

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::{action_cancel_reason::ActionCancelReasonV20230925, terminal_action_action_type::TerminalActionActionTypeV20230925};

use super::{qr_code_options::QrCodeOptionsV20230925, save_card_options::SaveCardOptionsV20230925, signature_options::SignatureOptionsV20230925, confirmation_options::ConfirmationOptionsV20230925, receipt_options::ReceiptOptionsV20230925, data_collection_options::DataCollectionOptionsV20230925, select_options::SelectOptionsV20230925, device_metadata::DeviceMetadataV20230925};

/// Represents an action processed by the Square Terminal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalActionV20230925 {
    /// Read only A unique ID for this TerminalAction.
    ///
    /// Min Length 10 Max Length 255
    pub id: Option<String>,
    /// The unique Id of the device intended for this TerminalAction. The Id can be retrieved from /v2/devices api.
    pub device_id: Option<String>,
    /// The duration as an RFC 3339 duration, after which the action will be automatically canceled. TerminalActions that are PENDING will be automatically CANCELED and have a cancellation reason of TIMED_OUT
    ///
    /// Default: 5 minutes from creation
    ///
    /// Maximum: 5 minutes
    ///
    /// Example for 2 days, 12 hours, 30 minutes, and 15 seconds: P2DT12H30M15S
    pub deadline_duration: Option<String>,
    /// Read only The status of the TerminalAction. Options: PENDING, IN_PROGRESS, CANCEL_REQUESTED, CANCELED, COMPLETED
    pub status: Option<String>,
    /// Read only The reason why TerminalAction is canceled. Present if the status is CANCELED.
    pub cancel_reason: Option<ActionCancelReasonV20230925>,
    /// Read only The time when the TerminalAction was created as an RFC 3339 timestamp.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// Read only The time when the TerminalAction was last updated as an RFC 3339 timestamp.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
    /// Read only The ID of the application that created the action.
    pub app_id: Option<String>,
    /// Read only The location id the action is attached to, if a link can be made.
    ///
    /// Max Length 64
    pub location_id: Option<String>,
    /// Represents the type of the action.
    pub r#type: Option<TerminalActionActionTypeV20230925>,
    /// Describes configuration for the QR code action. Requires QR_CODE type.
    pub qr_code_options: Option<QrCodeOptionsV20230925>,
    /// Describes configuration for the save-card action. Requires SAVE_CARD type.
    pub save_card_options: Option<SaveCardOptionsV20230925>,
    /// Describes configuration for the signature capture action. Requires SIGNATURE type.
    pub signature_options: Option<SignatureOptionsV20230925>,
    /// Describes configuration for the confirmation action. Requires CONFIRMATION type.
    pub confirmation_options: Option<ConfirmationOptionsV20230925>,
    /// Describes configuration for the receipt action. Requires RECEIPT type.
    pub receipt_options: Option<ReceiptOptionsV20230925>,
    /// Describes configuration for the data collection action. Requires DATA_COLLECTION type.
    pub data_collection_options: Option<DataCollectionOptionsV20230925>,
    /// Describes configuration for the select action. Requires SELECT type.
    pub select_options: Option<SelectOptionsV20230925>,
    /// Read only Details about the Terminal that received the action request (such as battery level, operating system version, and network connection settings).
    pub device_metadata: Option<DeviceMetadataV20230925>,
    /// Indicates the action will be linked to another action and requires a waiting dialog to be displayed instead of returning to the idle screen on completion of the action.
    ///
    /// Only supported on SIGNATURE, CONFIRMATION, DATA_COLLECTION, and SELECT types.
    pub await_next_action: Option<bool>,
    /// The timeout duration of the waiting dialog as an RFC 3339 duration, after which the waiting dialog will no longer be displayed and the Terminal will return to the idle screen.
    ///
    /// Default: 5 minutes from when the waiting dialog is displayed
    ///
    /// Maximum: 5 minutes
    ///
    /// Example for 2 days, 12 hours, 30 minutes, and 15 seconds: P2DT12H30M15S
    pub await_next_action_duration: Option<String>,
}
