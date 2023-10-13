//! TerminalActionActionType Enum

use serde::{Deserialize, Serialize};

/// Describes the type of this unit and indicates which field contains the unit information.
///
/// This is an 'open' enum.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TerminalActionActionTypeV20230925 {
    /// The action represents a request to display a QR code. Details are contained in the qr_code_options object.
    QrCode,
    /// The action represents a request to check if the specific device is online or currently active with the merchant in question. Does not require an action options value.
    Ping,
    /// Represents a request to save a card for future card-on-file use. Details are contained in the save_card_options object.
    SaveCard,
    /// The action represents a request to capture a buyer's signature. Details are contained in the signature_options object.
    Signature,
    /// The action represents a request to collect a buyer's confirmation decision to the displayed terms. Details are contained in the confirmation_options object.
    Confirmation,
    /// The action represents a request to display the receipt screen options. Details are contained in the receipt_options object.
    Receipt,
    /// The action represents a request to collect a buyer's text data. Details are contained in the data_collection_options object.
    DataCollection,
    /// The action represents a request to allow the buyer to select from provided options. Details are contained in the select_options object.
    Select,
}
