//! GiftCardGANSource Enum

use serde::{Deserialize, Serialize};

/// Indicates the source that generated the gift card account number (GAN).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GiftCardGANSource {
    /// The GAN is generated by Square.
    Square,
    /// The GAN is provided by a non-Square system. For more information, see [Custom GANs](https://developer.squareup.com/docs/gift-cards/using-gift-cards-api#custom-gans) or [Third-party gift cards](https://developer.squareup.com/docs/gift-cards/using-gift-cards-api#third-party-gift-cards).
    Other,
}
