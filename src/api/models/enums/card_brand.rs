//! CardBrand Enum

use serde::{Deserialize, Serialize};

/// Indicates a card's brand, such as VISA or MASTERCARD.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardBrand {
    /// A card brand not on this list.
    OtherBrand,
    /// VISA.
    Visa,
    /// MASTERCARD.
    Mastercard,
    /// AMERICAN_EXPRESS.
    AmericanExpress,
    /// DISCOVER.
    Discover,
    /// DISCOVER_DINERS.
    DiscoverDiners,
    /// JCB.
    Jcb,
    /// CHINA_UNIONPAY.
    ChinaUnionpay,
    /// SQUARE_GIFT_CARD.
    SquareGiftCard,
    /// SQUARE_CAPITAL_CARD.
    SquareCapitalCard,
    /// INTERAC.
    Interac,
    /// EFTPOS.
    Eftpos,
    /// FELICA.
    Felica,
    /// EBT.
    Ebt,
}
